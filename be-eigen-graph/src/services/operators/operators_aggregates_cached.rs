use crate::models::cached::{Cached, DataSource};
use crate::models::operators_aggr::UniformPage;
use crate::models::operators_snapshot::{OperatorOrderBy, OperatorsSnapshotVars, OrderDirection};
use crate::services::operators::operators_aggr::from_subgraph_adapt::from_subgraph_adapt;
use crate::services::operators::operators_snapshot_cached::operators_snapshot_cached;
use redis::aio::ConnectionManager;
use reqwest::{Client, Url};

pub async fn uniform_page_from_subgraph_cached(
    client: Client,
    endpoint: Url,
    first: i32,
    skip: i32,
    redis: &Option<ConnectionManager>,
    ttl_secs: u64,
) -> Result<Cached<UniformPage>, anyhow::Error> {
    let vars = OperatorsSnapshotVars {
        first,
        skip,
        order_by: OperatorOrderBy::LastUpdateBlockTimestamp,
        order_direction: OrderDirection::Desc,
        has_slashing: 0,
    };
    let cached = operators_snapshot_cached(client, endpoint, vars, redis, ttl_secs).await?;
    let page: UniformPage = from_subgraph_adapt(&cached.data, first, skip);
    let source = match cached.source {
        crate::models::cached::DataSource::Redis => DataSource::Redis,
        _ => DataSource::Subgraph,
    };
    Ok(Cached { source, data: page })
}
