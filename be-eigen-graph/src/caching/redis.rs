use crate::models::operators_snapshot::{OperatorOrderBy, OperatorsSnapshotVars, OrderDirection};
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub fn key_snapshot(vars: &OperatorsSnapshotVars) -> String {
    let ob = match vars.order_by {
        OperatorOrderBy::Id => "id",
        OperatorOrderBy::AvsCount => "avsCount",
        OperatorOrderBy::StrategyCount => "strategyCount",
        OperatorOrderBy::SlashingCount => "slashingCount",
        OperatorOrderBy::LastUpdateBlockTimestamp => "lastUpdateBlockTimestamp",
    };
    let od = match vars.order_direction {
        OrderDirection::Asc => "asc",
        OrderDirection::Desc => "desc",
    };
    format!(
        "snapshot:first={}:skip={}:orderBy={}:orderDir={}:hasSlashing={}",
        vars.first, vars.skip, ob, od, vars.has_slashing
    )
}

pub fn key_snapshot_page(first: i32, skip: i32) -> String {
    format!("snapshot_page:first={first}:skip={skip}")
}

pub async fn get_json<T: DeserializeOwned>(conn: &mut ConnectionManager, key: &str) -> Option<T> {
    let bytes: Option<Vec<u8>> = conn.get(key).await.ok();
    bytes.and_then(|b| serde_json::from_slice(&b).ok())
}

pub async fn set_json<T: Serialize>(
    conn: &mut ConnectionManager,
    key: &str,
    value: &T,
    ttl_secs: u64,
) -> Result<(), redis::RedisError> {
    let buf = serde_json::to_vec(value)
        .map_err(|_| redis::RedisError::from((redis::ErrorKind::TypeError, "serialize")))?;
    conn.set_ex(key, buf, ttl_secs as usize as u64).await
}
