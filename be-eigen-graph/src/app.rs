use crate::api::subgraph::client::SubgraphClient;
use crate::config::AppConfig;
use crate::routes::v1;
use crate::state::AppState;
use axum::Router;
use axum::http::Method;
use redis::Client;
use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

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

    let redis = match Client::open(config.redis_url.as_str()) {
        Ok(client) => ConnectionManager::new(client).await.ok(),
        Err(_) => None,
    };

    let state = AppState {
        subgraph_client: SubgraphClient::new(config.subgraph_url),
        operators_snapshot: Arc::new(Mutex::new(HashMap::new())),
        db,
        redis,
        redis_ttl_seconds: config.redis_ttl_seconds,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    Router::new()
        .nest("/v1", v1::routes())
        .layer(cors)
        .with_state(state)
}
