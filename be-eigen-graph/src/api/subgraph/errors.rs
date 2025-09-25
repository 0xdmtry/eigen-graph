use thiserror::Error;

use crate::models::subgraph::GraphQLError;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("decode error: {0}")]
    Decode(#[from] serde_json::Error),

    #[error("graphql errors: {0:?}")]
    GraphQL(Vec<GraphQLError>),

    #[error("empty graphql data payload")]
    EmptyData,
}
