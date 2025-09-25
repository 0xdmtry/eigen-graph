use crate::handlers::operators_handler::snapshot_handler;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/operators/snapshot", get(snapshot_handler))
}
