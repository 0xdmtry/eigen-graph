use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::stream::state::StreamState;
use crate::stream::ws;

pub async fn run_ws_server(bind_addr: &str, stream_state: Arc<StreamState>) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/stream/deposits", get(ws::deposits_ws_handler))
        .with_state(stream_state);

    let listener = TcpListener::bind(bind_addr).await?;
    let addr: SocketAddr = listener.local_addr()?;
    println!("[stream] WS server listening on {addr}");

    axum::serve(listener, router).await?;
    Ok(())
}

pub async fn run_poller(stream_state: Arc<StreamState>) {
    let _ = stream_state;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
