use crate::models::cached::Cached;
use crate::models::operators_snapshot::{OperatorOrderBy, OperatorsSnapshotVars, OrderDirection};
use crate::payloads::operators::SnapshotQuery;
use crate::services::operators::operators_cache::upsert_operators_snapshot_cache;
use crate::services::operators::operators_repo::persist_operators_snapshot_db;
use crate::services::operators::operators_snapshot_cached::operators_snapshot_cached;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};

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
