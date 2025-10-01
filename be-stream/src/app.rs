use crate::config::AppConfig;
use crate::routes::v1;
use crate::services::coinbase;
use crate::state::AppState;
use axum::Router;
use axum::http::Method;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
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

        // sqlx::migrate!()
        //     .run(&pool)
        //     .await
        //     .expect("timescale migrations failed");

        Some(pool)
    } else {
        None
    };

    coinbase::spawn_coinbase_client(config.clone());

    let state = AppState { config, ts_db };

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
