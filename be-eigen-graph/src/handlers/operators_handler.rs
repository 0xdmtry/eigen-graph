use crate::metrics::error_inc;
use crate::models::operators_aggr::AggregatorParams;
use crate::models::operators_snapshot::{OperatorOrderBy, OperatorsSnapshotVars, OrderDirection};
use crate::payloads::operators::SnapshotQuery;
use crate::payloads::operators::TokenSlice;
use crate::payloads::operators::{AggregatesMeta, AggregatesQuery, AggregatesResponse};
use crate::services::operators::operators_aggr::operators_part::partition_by_token;
use crate::services::operators::operators_aggr::{
    from_db_adapt, from_subgraph_adapt, operators_aggregator,
};
use crate::services::operators::operators_fetcher::operators_snapshot;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use std::collections::BTreeMap;

pub async fn snapshot_handler(
    State(state): State<AppState>,
    Query(q): Query<SnapshotQuery>,
) -> impl IntoResponse {
    let vars = OperatorsSnapshotVars {
        first: q.first.unwrap_or(25),
        skip: q.skip.unwrap_or(0),
        has_slashing: q.has_slashing.unwrap_or(0),
        order_by: parse_order_by(q.order_by.as_deref())
            .unwrap_or(OperatorOrderBy::LastUpdateBlockTimestamp),
        order_direction: match q.order_direction.as_deref() {
            Some("asc") => OrderDirection::Asc,
            _ => OrderDirection::Desc,
        },
    };

    let data = operators_snapshot(
        state.subgraph_client.http.clone(),
        state.subgraph_client.endpoint.clone(),
        vars,
    )
    .await
    .expect("subgraph query failed");

    if let Err(_e) =
        crate::services::operators::operators_repo::persist_operators_snapshot_db(&state.db, &data)
            .await
    {
        error_inc("persist_operators_snapshot_db");
    }

    crate::services::operators::operators_cache::upsert_operators_snapshot_cache(
        &state.operators_snapshot,
        &data,
    );

    Json(data)
}

pub async fn operators_aggregates_handler(
    State(state): State<AppState>,
    Query(q): Query<AggregatesQuery>,
) -> impl IntoResponse {
    let source = q.source.as_deref().unwrap_or("live");
    let first = q.first.unwrap_or(25);
    let skip = q.skip.unwrap_or(0);

    let params = AggregatorParams {
        top_n: q.top_n.unwrap_or(10).clamp(1, 100),
        hhi_threshold: q.hhi_threshold.unwrap_or(0.2),
        recent_window_s: 7 * 24 * 3600,
        min_tvl_atomic: q.min_tvl_atomic.clone(),
        focus_operator_id: q.operator_id.clone(),
    };

    let uniform = match source {
        "db" => {
            // DB adapter
            from_db_adapt::from_db_adapt(&state.db, first, skip)
                .await
                .unwrap_or_else(|_| crate::models::operators_aggr::UniformPage {
                    operators: vec![],
                    page_meta: crate::models::operators_aggr::PageMeta { first, skip },
                })
        }
        _ => {
            let vars = OperatorsSnapshotVars {
                first,
                skip,
                has_slashing: 0,
                order_by: OperatorOrderBy::LastUpdateBlockTimestamp,
                order_direction: OrderDirection::Desc,
            };
            let page = operators_snapshot(
                state.subgraph_client.http.clone(),
                state.subgraph_client.endpoint.clone(),
                vars,
            )
            .await
            .expect("subgraph query failed");

            from_subgraph_adapt::from_subgraph_adapt(&page, first, skip)
        }
    };

    let now_ts = chrono::Utc::now().timestamp();
    let aggregates = operators_aggregator::aggregate(&uniform, &params, now_ts);

    let table = operators_aggregator::to_table_rows(&aggregates);
    let bar = operators_aggregator::to_bar_series(&aggregates, params.top_n);
    let donuts_vec = operators_aggregator::to_donuts(&aggregates, &params.focus_operator_id);
    let donut = if params.focus_operator_id.is_some() {
        if let Some(d) = donuts_vec.into_iter().next() {
            serde_json::to_value(d).unwrap_or(serde_json::json!({}))
        } else {
            serde_json::json!({})
        }
    } else {
        let map = donuts_vec
            .into_iter()
            .map(|d| (d.operator_id.clone(), d))
            .collect::<std::collections::BTreeMap<_, _>>();
        serde_json::to_value(map).unwrap_or(serde_json::json!({}))
    };
    let graph = operators_aggregator::to_graph_edges(&aggregates);
    let outliers = operators_aggregator::detect_outliers(
        &aggregates,
        params.hhi_threshold,
        params.recent_window_s,
        now_ts,
    );

    let mut by_token: BTreeMap<String, TokenSlice> = BTreeMap::new();
    let token_pages = partition_by_token(&uniform);
    for (symbol, token_page) in token_pages {
        let aggr_tok = operators_aggregator::aggregate(&token_page, &params, now_ts);

        let table_tok = operators_aggregator::to_table_rows(&aggr_tok);
        let bar_tok = operators_aggregator::to_bar_series(&aggr_tok, params.top_n);
        let donuts_tok = operators_aggregator::to_donuts(&aggr_tok, &params.focus_operator_id);
        let donut_tok = if params.focus_operator_id.is_some() {
            if let Some(d) = donuts_tok.into_iter().next() {
                serde_json::to_value(d).unwrap_or(serde_json::json!({}))
            } else {
                serde_json::json!({})
            }
        } else {
            let map = donuts_tok
                .into_iter()
                .map(|d| (d.operator_id.clone(), d))
                .collect::<BTreeMap<_, _>>();
            serde_json::to_value(map).unwrap_or(serde_json::json!({}))
        };
        let graph_tok = operators_aggregator::to_graph_edges(&aggr_tok);
        let out_tok = operators_aggregator::detect_outliers(
            &aggr_tok,
            params.hhi_threshold,
            params.recent_window_s,
            now_ts,
        );

        let meta_tok = AggregatesMeta {
            source: source.to_string(),
            first,
            skip,
            count: aggr_tok.len(),
        };

        by_token.insert(
            symbol,
            TokenSlice {
                meta: meta_tok,
                table: table_tok,
                bar: bar_tok,
                donut: donut_tok,
                graph: graph_tok,
                outliers: out_tok,
            },
        );
    }

    let resp = AggregatesResponse {
        meta: AggregatesMeta {
            source: source.to_string(),
            first,
            skip,
            count: aggregates.len(),
        },
        table,
        bar,
        donut,
        graph,
        outliers,
        by_token,
    };

    Json(resp)
}

fn parse_order_by(s: Option<&str>) -> Option<OperatorOrderBy> {
    match s? {
        "id" => Some(OperatorOrderBy::Id),
        "avsCount" | "avs_count" => Some(OperatorOrderBy::AvsCount),
        "strategyCount" | "strategy_count" => Some(OperatorOrderBy::StrategyCount),
        "slashingCount" | "slashing_count" => Some(OperatorOrderBy::SlashingCount),
        "lastUpdateBlockTimestamp" | "last_update_block_timestamp" => {
            Some(OperatorOrderBy::LastUpdateBlockTimestamp)
        }
        _ => None,
    }
}
