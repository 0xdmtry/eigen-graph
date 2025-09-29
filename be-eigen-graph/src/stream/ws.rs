use std::sync::Arc;

use crate::stream::state::{Event, StreamState};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use tokio::sync::broadcast;

use chrono::Utc;
use sqlx::postgres::PgPoolOptions;

use crate::api::subgraph::client::SubgraphClient;
use crate::config::AppConfig;
use crate::stream::db::load_cursor;
use crate::stream::db::{BucketPoint, fetch_buckets};
use crate::stream::subgraph::resolve_token_id;

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
    let window_sec = env_i64("DEPOSITS_WINDOW_SEC", 86_400); // 24h
    let bucket_sec = env_i64("DEPOSITS_BUCKET_SEC", 300); // 5m
    let poll_every = env_u64("DEPOSITS_POLL_EVERY_MS", 3_000);

    let cfg = AppConfig::from_env();
    let sg = SubgraphClient::new(cfg.subgraph_url);
    let (token_id, token_label) = match resolve_token_id(&sg, &q.token).await {
        Ok(x) => x,
        Err(e) => {
            let err =
                serde_json::json!({"type":"error","reason":format!("token resolve failed: {e}")});
            let _ = socket
                .send(Message::Text(Utf8Bytes::from(err.to_string())))
                .await;
            return;
        }
    };

    let ts_url = match cfg.timescale_database_url {
        Some(u) => u,
        None => {
            let err = serde_json::json!({"type":"error","reason":"TIMESCALE_DATABASE_URL not set"});
            let _ = socket
                .send(Message::Text(Utf8Bytes::from(err.to_string())))
                .await;
            return;
        }
    };
    let ts_pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&ts_url)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            let err = serde_json::json!({"type":"error","reason":format!("ts pool failed: {e}")});
            let _ = socket
                .send(Message::Text(Utf8Bytes::from(err.to_string())))
                .await;
            return;
        }
    };

    let now = Utc::now().timestamp();
    let start_ts = now.saturating_sub(window_sec);
    let end_ts = now;
    let mut series = match fetch_buckets(&ts_pool, &token_id, start_ts, bucket_sec).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[stream] fetch_buckets err: {e}");
            Vec::new()
        }
    };

    series = zero_fill(series, start_ts, end_ts, bucket_sec);

    stream_state.register_interest(&q.token, Some(start_ts));

    let hello = serde_json::json!({
      "type":"hello",
      "token": q.token,
      "token_id": token_id,
      "token_label": token_label,
      "poll_every_ms": poll_every,
      "window_sec": window_sec,
      "bucket_sec": bucket_sec,
      "subscribers": stream_state.subscriber_count(&q.token)
    });
    let _ = socket
        .send(Message::Text(Utf8Bytes::from(hello.to_string())))
        .await;

    let _last_bucket_ts = series.last().map(|p| p.t).unwrap_or(start_ts);
    let persisted_cursor = match load_cursor(&ts_pool, &token_id).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[stream] load_cursor in ws err: {e}");
            None
        }
    };
    let (cursor_ts, cursor_id) = persisted_cursor.unwrap_or((
        series.last().map(|p| p.t).unwrap_or(start_ts),
        String::new(),
    ));

    let init = serde_json::json!({
      "type":"init",
      "token": q.token,
      "token_id": token_id,
      "window_sec": window_sec,
      "bucket_sec": bucket_sec,
      "series": series.iter().map(|p| serde_json::json!({"t": p.t, "count": p.count, "sum_shares": p.sum_shares})).collect::<Vec<_>>(),
      "cursor": { "last_ts": cursor_ts, "last_id": cursor_id }
    });
    let _ = socket
        .send(Message::Text(Utf8Bytes::from(init.to_string())))
        .await;

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

fn env_i64(name: &str, default: i64) -> i64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn zero_fill(mut points: Vec<BucketPoint>, start: i64, end: i64, step: i64) -> Vec<BucketPoint> {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<i64, (i64, String)> = BTreeMap::new();
    for p in points.drain(..) {
        map.insert(p.t, (p.count, p.sum_shares));
    }
    let mut out = Vec::new();
    let mut t = (start / step) * step;
    while t <= end {
        if let Some((c, s)) = map.get(&t) {
            out.push(BucketPoint {
                t,
                count: *c,
                sum_shares: s.clone(),
            });
        } else {
            out.push(BucketPoint {
                t,
                count: 0,
                sum_shares: "0".into(),
            });
        }
        t += step;
    }
    out
}
