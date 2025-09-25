use crate::api::subgraph::client::SubgraphClient;
use crate::config::AppConfig;
use crate::routes::v1;
use crate::state::AppState;
use axum::Router;

pub fn app() -> Router {
    let config = AppConfig::from_env();
    let state = AppState {
        subgraph_client: SubgraphClient::new(config.subgraph_url),
    };

    Router::new().nest("/v1", v1::routes()).with_state(state)
}
