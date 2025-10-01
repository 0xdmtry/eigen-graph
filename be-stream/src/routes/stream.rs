use axum::extract::ws::Utf8Bytes;
use axum::{
    Router,
    extract::ws::{Message, WebSocket},
    extract::{Query, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;

use crate::{models::tick::Tick, state::AppState};

#[derive(Deserialize)]
struct StreamQuery {
    symbol: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/stream/ws", get(ws_route))
}

async fn ws_route(
    State(state): State<AppState>,
    Query(q): Query<StreamQuery>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state, q.symbol))
}

async fn handle_ws(mut socket: WebSocket, state: AppState, symbol: String) {
    let tx = {
        let map = state.broadcasters.read().await;
        map.get(&symbol).cloned()
    };

    let mut rx = if let Some(tx) = tx {
        tx.subscribe()
    } else {
        let (tx_new, rx_new) = tokio::sync::broadcast::channel::<Tick>(1024);
        {
            let mut map = state.broadcasters.write().await;
            map.entry(symbol.clone()).or_insert(tx_new);
        }
        rx_new
    };

    while let Ok(tick) = rx.recv().await {
        if tick.product_id != symbol {
            continue;
        }
        if socket
            .send(Message::Text(Utf8Bytes::from(
                serde_json::to_string(&tick).unwrap(),
            )))
            .await
            .is_err()
        {
            break;
        }
    }
}
