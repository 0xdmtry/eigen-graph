use crate::api::subgraph::client::SubgraphClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub subgraph_client: SubgraphClient,
}
