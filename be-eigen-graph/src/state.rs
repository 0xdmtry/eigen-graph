use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::api::subgraph::client::SubgraphClient;
use crate::models::operators_snapshot::OperatorDto;

#[derive(Debug, Clone)]
pub struct AppState {
    pub subgraph_client: SubgraphClient,
    pub operators_snapshot: Arc<Mutex<HashMap<String, OperatorDto>>>
}
