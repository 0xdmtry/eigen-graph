use axum::{Router, response::IntoResponse, routing::get, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn ping() -> impl IntoResponse {
    "pong"
}

fn app() -> Router {
    Router::new().route("/", get(ping))
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Cannot bind port 8000");
    let app = app().into_make_service_with_connect_info::<SocketAddr>();

    serve(listener, app).await.expect("Cannot serve");
}
