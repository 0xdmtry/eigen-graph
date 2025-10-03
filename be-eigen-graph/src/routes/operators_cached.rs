use crate::handlers::operators_cached_handler::snapshot_cached_handler;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/operators/snapshot_cached", get(snapshot_cached_handler))
}
