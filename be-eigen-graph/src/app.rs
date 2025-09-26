use crate::api::subgraph::client::SubgraphClient;
use crate::config::AppConfig;
use crate::routes::v1;
use crate::state::AppState;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub async fn app() -> Router {
    let config = AppConfig::from_env();
    let db = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .expect("failed to connect to Postgres");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("db migrations failed");

    let state = AppState {
        subgraph_client: SubgraphClient::new(config.subgraph_url),
        operators_snapshot: Arc::new(Mutex::new(HashMap::new())),
        db,
    };

    Router::new().nest("/v1", v1::routes()).with_state(state)
}
