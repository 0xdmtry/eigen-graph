use crate::models::operators_snapshot::{OperatorOrderBy, OperatorsSnapshotVars, OrderDirection};
use crate::payloads::operators::SnapshotQuery;
use crate::services::operators::operators_fetcher::operators_snapshot;

use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract::Query, response::Json};

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

    Json(data)
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
