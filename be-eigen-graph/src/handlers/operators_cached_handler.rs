use crate::models::cached::Cached;
use crate::models::cached::DataSource;
use crate::models::operators_aggr::UniformPage;
use crate::models::operators_snapshot::{OperatorOrderBy, OperatorsSnapshotVars, OrderDirection};
use crate::payloads::operators::SnapshotQuery;
use crate::payloads::operators::{AggregatesMeta, AggregatesQuery, AggregatesResponse, TokenSlice};
use crate::services::operators::operators_aggr::{
    from_db_adapt, operators_aggregator, operators_part::partition_by_token,
};
use crate::services::operators::operators_aggregates_cached::uniform_page_from_subgraph_cached;
use crate::services::operators::operators_cache::upsert_operators_snapshot_cache;
use crate::services::operators::operators_repo::persist_operators_snapshot_db;
use crate::services::operators::operators_snapshot_cached::operators_snapshot_cached;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use std::collections::BTreeMap;

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

pub async fn snapshot_cached_handler(
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

    let cached: Cached<crate::models::operators_snapshot::OperatorsSnapshotData> =
        operators_snapshot_cached(
            state.subgraph_client.http.clone(),
            state.subgraph_client.endpoint.clone(),
            vars,
            &state.redis,
            state.redis_ttl_seconds,
        )
        .await
        .expect("snapshot cached failed");

    if matches!(cached.source, crate::models::cached::DataSource::Subgraph) {
        let _ = persist_operators_snapshot_db(&state.db, &cached.data).await;
        upsert_operators_snapshot_cache(&state.operators_snapshot, &cached.data);
    }

    Json(cached)
}

pub async fn operators_aggregates_cached_handler(
    State(state): State<crate::state::AppState>,
    Query(q): Query<AggregatesQuery>,
) -> impl IntoResponse {
    let source = q.source.as_deref().unwrap_or("db");
    let first = q.first.unwrap_or(25);
    let skip = q.skip.unwrap_or(0);

    let params = crate::models::operators_aggr::AggregatorParams {
        top_n: q.top_n.unwrap_or(10).clamp(1, 100),
        hhi_threshold: q.hhi_threshold.unwrap_or(0.2),
        recent_window_s: 7 * 24 * 3600,
        min_tvl_atomic: q.min_tvl_atomic.clone(),
        focus_operator_id: q.operator_id.clone(),
    };

    let cached_page: Cached<UniformPage> = match source {
        "db" => {
            let page = from_db_adapt::from_db_adapt(&state.db, first, skip)
                .await
                .unwrap_or(crate::models::operators_aggr::UniformPage {
                    operators: vec![],
                    page_meta: crate::models::operators_aggr::PageMeta { first, skip },
                });
            Cached {
                source: DataSource::Db,
                data: page,
            }
        }
        _ => uniform_page_from_subgraph_cached(
            state.subgraph_client.http.clone(),
            state.subgraph_client.endpoint.clone(),
            first,
            skip,
            &state.redis,
            state.redis_ttl_seconds,
        )
        .await
        .expect("uniform_page_from_subgraph_cached failed"),
    };

    let now_ts = chrono::Utc::now().timestamp();
    let aggregates = operators_aggregator::aggregate(&cached_page.data, &params, now_ts);

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
    let token_pages = partition_by_token(&cached_page.data);
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

    Json(Cached {
        source: cached_page.source,
        data: resp,
    })
}
