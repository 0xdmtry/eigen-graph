use super::{ping, stream};
use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().merge(ping::routes()).merge(stream::routes())
}
