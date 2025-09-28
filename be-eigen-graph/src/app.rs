use crate::api::subgraph::client::SubgraphClient;
use crate::config::AppConfig;
use crate::routes::v1;
use crate::state::AppState;
use axum::Router;
use axum::http::Method;
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

    let ts_db = if let Some(ts_url) = &config.timescale_database_url {

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(ts_url)
            .await
            .expect("failed to connect to TimescaleDB");

        sqlx::migrate!("./migrations_timescale")
            .run(&pool)
            .await
            .expect("timescale migrations failed");

        Some(pool)
    } else {
        None
    };

    let state = AppState {
        subgraph_client: SubgraphClient::new(config.subgraph_url),
        operators_snapshot: Arc::new(Mutex::new(HashMap::new())),
        db,
        ts_db,
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
