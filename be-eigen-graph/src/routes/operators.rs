use crate::handlers::operators_handler::{operators_aggregates_handler, snapshot_handler};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/operators/snapshot", get(snapshot_handler))
        .route("/operators/aggregates", get(operators_aggregates_handler))
}
