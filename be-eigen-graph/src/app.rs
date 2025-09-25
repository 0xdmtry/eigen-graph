use crate::routes::v1;
use axum::Router;

pub fn app() -> Router {
    Router::new().nest("/v1", v1::routes())
}
