use crate::metrics::cache_inc;
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
    let res: Result<Option<Vec<u8>>, _> = conn.get(key).await;
    match res {
        Ok(Some(b)) => {
            let v = serde_json::from_slice(&b).ok();
            cache_inc("get", "hit");
            v
        }
        Ok(None) => {
            cache_inc("get", "miss");
            None
        }
        Err(_) => {
            cache_inc("get", "err");
            None
        }
    }
}

pub async fn set_json<T: Serialize>(
    conn: &mut ConnectionManager,
    key: &str,
    value: &T,
    ttl_secs: u64,
) -> Result<(), redis::RedisError> {
    let buf = serde_json::to_vec(value)
        .map_err(|_| redis::RedisError::from((redis::ErrorKind::TypeError, "serialize")))?;
    let res = conn.set_ex(key, buf, ttl_secs as usize as u64).await;
    match &res {
        Ok(_) => cache_inc("set", "ok"),
        Err(_) => cache_inc("set", "err"),
    }
    res
}
