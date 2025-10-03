use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataSource {
    Redis,
    Subgraph,
    Db,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cached<T> {
    pub source: DataSource,
    pub data: T,
}
