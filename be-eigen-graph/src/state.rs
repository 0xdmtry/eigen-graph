use crate::api::subgraph::client::SubgraphClient;
use crate::models::operators_snapshot::OperatorDto;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    pub subgraph_client: SubgraphClient,
    pub operators_snapshot: Arc<Mutex<HashMap<String, OperatorDto>>>,
    pub db: sqlx::Pool<sqlx::Postgres>,
    pub ts_db: Option<sqlx::Pool<sqlx::Postgres>>,
}
