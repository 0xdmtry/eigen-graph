use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct GraphQLRequest<'q, V> {
    pub query: &'q str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<V>,
}

#[derive(Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    #[serde(default)]
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLError {
    pub message: String,
    #[serde(default)]
    pub path: Option<Vec<Value>>,
    #[serde(default)]
    pub locations: Option<Vec<GraphQLErrorLocation>>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLErrorLocation {
    pub line: i32,
    pub column: i32,
}
