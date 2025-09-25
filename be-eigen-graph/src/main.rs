use axum::serve;
use be_eigen_graph::app::app;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Cannot bind port 8000");
    let app = app().into_make_service_with_connect_info::<SocketAddr>();

    serve(listener, app).await.expect("Cannot serve");
}
