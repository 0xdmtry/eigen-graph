use axum::serve;
use be_eigen_graph::app::app;
use be_eigen_graph::stream::runner::{run_poller, run_ws_server};
use be_eigen_graph::stream::state::StreamState;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Cannot bind port 8000");
    let app = app()
        .await
        .into_make_service_with_connect_info::<SocketAddr>();

    tokio::spawn(async move {
        let stream_state = Arc::new(StreamState::new());

        let ws_srv = tokio::spawn({
            let ss = stream_state.clone();
            async move {
                if let Err(e) = run_ws_server("0.0.0.0:8010", ss).await {
                    eprintln!("[stream] ws server error: {e:?}");
                }
            }
        });

        let poller = tokio::spawn({
            let ss = stream_state.clone();
            async move {
                run_poller(ss).await;
            }
        });

        let _ = tokio::join!(ws_srv, poller);
    });

    serve(listener, app).await.expect("Cannot serve");
}
