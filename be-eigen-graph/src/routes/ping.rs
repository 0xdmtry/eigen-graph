use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/ping", get(|| async { "pong" }))
}
