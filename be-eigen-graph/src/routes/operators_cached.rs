use crate::handlers::operators_cached_handler::{
    operators_aggregates_cached_handler, snapshot_cached_handler,
};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/operators/snapshot_cached", get(snapshot_cached_handler))
        .route(
            "/operators/aggregates_cached",
            get(operators_aggregates_cached_handler),
        )
}
