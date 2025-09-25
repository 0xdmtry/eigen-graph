use crate::api::subgraph::errors::InfraError;
use crate::api::subgraph::queries::OPERATORS_SNAPSHOT;
use crate::models::operators_snapshot::{OperatorsSnapshotData, OperatorsSnapshotVars};
use crate::models::subgraph::{GraphQLRequest, GraphQLResponse};
use reqwest::{Client, Url};

pub async fn operators_snapshot(
    client: Client,
    endpoint: Url,
    vars: OperatorsSnapshotVars,
) -> Result<OperatorsSnapshotData, InfraError> {
    let body = GraphQLRequest {
        query: OPERATORS_SNAPSHOT,
        variables: Some(vars),
    };

    let resp = client
        .post(endpoint.clone())
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json::<GraphQLResponse<OperatorsSnapshotData>>()
        .await?;

    if let Some(errs) = resp.errors {
        return Err(InfraError::GraphQL(errs));
    }
    let data = resp.data.ok_or(InfraError::EmptyData)?;
    Ok(data)
}
