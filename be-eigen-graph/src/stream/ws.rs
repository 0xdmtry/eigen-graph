use std::sync::Arc;

use crate::stream::state::{Event, StreamState};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use tokio::sync::broadcast;

#[derive(Debug, serde::Deserialize)]
pub struct StreamQuery {
    pub token: String,
    pub since: Option<i64>,
}

pub async fn deposits_ws_handler(
    State(stream_state): State<Arc<StreamState>>,
    Query(q): Query<StreamQuery>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, stream_state, q))
}

async fn handle_ws(mut socket: WebSocket, stream_state: Arc<StreamState>, q: StreamQuery) {
    let mut rx = stream_state.subscribe(&q.token);

    let hello = format!(
        r#"{{"type":"hello","token":"{}","since":{},"subscribers":{}}}"#,
        q.token,
        q.since.unwrap_or_default(),
        stream_state.subscriber_count(&q.token)
    );
    let _ = socket.send(Message::Text(Utf8Bytes::from(hello))).await;

    stream_state.register_interest(&q.token, q.since);

    loop {
        tokio::select! {
            biased;

            evt = rx.recv() => {
                match evt {
                    Ok(Event(s)) => {
                        if socket.send(Message::Text(Utf8Bytes::from(s))).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        let _ = socket.send(Message::Text(r#"{"type":"warning","reason":"lagged"}"#.into())).await;
                    }
                }
            }

            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {  }
                }
            }
        }
    }
}
