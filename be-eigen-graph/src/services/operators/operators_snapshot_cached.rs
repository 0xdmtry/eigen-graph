use crate::api::subgraph::queries::OPERATORS_SNAPSHOT;
use crate::caching::redis::{get_json, key_snapshot, set_json};
use crate::models::cached::{Cached, DataSource};
use crate::models::operators_snapshot::{OperatorsSnapshotData, OperatorsSnapshotVars};
use crate::models::subgraph::{GraphQLRequest, GraphQLResponse};
use redis::aio::ConnectionManager;
use reqwest::{Client, Url};

pub async fn operators_snapshot_cached(
    client: Client,
    endpoint: Url,
    vars: OperatorsSnapshotVars,
    redis: &Option<ConnectionManager>,
    ttl_secs: u64,
) -> Result<Cached<OperatorsSnapshotData>, anyhow::Error> {
    if let Some(rm) = redis.as_ref() {
        let key = key_snapshot(&vars);
        let mut conn = rm.clone();
        if let Some(hit) = get_json::<OperatorsSnapshotData>(&mut conn, &key).await {
            return Ok(Cached {
                source: DataSource::Redis,
                data: hit,
            });
        }
    }

    let body = GraphQLRequest {
        query: OPERATORS_SNAPSHOT,
        variables: Some(vars),
    };
    let resp = client
        .post(endpoint.clone())
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    let resp = resp
        .json::<GraphQLResponse<OperatorsSnapshotData>>()
        .await?;
    let data = resp.data.ok_or_else(|| anyhow::anyhow!("empty data"))?;

    if let Some(rm) = redis.as_ref() {
        let key = key_snapshot(body.variables.as_ref().unwrap());
        let mut conn = rm.clone();
        let _ = set_json(&mut conn, &key, &data, ttl_secs).await;
    }

    Ok(Cached {
        source: DataSource::Subgraph,
        data,
    })
}
