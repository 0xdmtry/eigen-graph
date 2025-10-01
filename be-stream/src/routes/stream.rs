use axum::extract::ws::Utf8Bytes;
use axum::{
    Router,
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use tokio::sync::broadcast;

use crate::{models::tick::Tick, services::coinbase::Control, state::AppState};

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
    let mut just_created = false;

    let tx = {
        let map = state.broadcasters.read().await;
        map.get(&symbol).cloned()
    };

    let tx = if let Some(tx) = tx {
        tx
    } else {
        let (tx_new, _rx) = broadcast::channel::<Tick>(1024);
        {
            let mut map = state.broadcasters.write().await;
            map.entry(symbol.clone()).or_insert(tx_new.clone());
        }
        just_created = true;
        tx_new
    };

    {
        let mut counts = state.sub_counts.write().await;
        let c = counts.entry(symbol.clone()).or_default();
        *c += 1;
        if *c == 1 {
            let _ = state
                .control_tx
                .send(Control::Subscribe(symbol.clone()))
                .await;
        }
    }

    let mut rx = tx.subscribe();

    loop {
        let tick = match rx.recv().await {
            Ok(t) => t,
            Err(_) => break,
        };
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

    {
        let mut counts = state.sub_counts.write().await;
        if let Some(c) = counts.get_mut(&symbol) {
            if *c > 0 {
                *c -= 1;
            }
            if *c == 0 {
                let _ = state
                    .control_tx
                    .send(Control::Unsubscribe(symbol.clone()))
                    .await;
            }
        }
    }

    if just_created {
        let mut map = state.broadcasters.write().await;
        if let Some(tx_current) = map.get(&symbol) {
            if tx_current.receiver_count() == 0 {
                map.remove(&symbol);
            }
        }
    }
}
