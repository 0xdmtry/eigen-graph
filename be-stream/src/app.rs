use crate::config::AppConfig;
use crate::routes::v1;
use crate::services::coinbase;
use crate::state::AppState;
use axum::Router;
use axum::http::Method;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

pub async fn app() -> Router {
    let config = AppConfig::from_env();

    let ts_db = if let Some(ts_url) = &config.timescale_database_url {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(ts_url)
            .await
            .expect("failed to connect to TimescaleDB");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("timescale migrations failed");

        Some(pool)
    } else {
        None
    };

    let (control_tx, control_rx) = mpsc::channel(64);

    let state = AppState {
        config: config.clone(),
        ts_db,
        broadcasters: Arc::new(RwLock::new(HashMap::new())),
        sub_counts: Arc::new(RwLock::new(HashMap::new())),
        control_tx,
    };

    coinbase::spawn_coinbase_client(state.clone(), control_rx);

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
