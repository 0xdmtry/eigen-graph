use crate::{models::tick::Tick, state::AppState};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::{
    sync::broadcast,
    time::{Duration, sleep},
};
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum CbMsg {
    #[serde(rename = "match")]
    Match {
        product_id: String,
        price: String,
        time: String,
    },
    #[serde(rename = "last_match")]
    LastMatch {
        product_id: String,
        price: String,
        time: String,
    },
    #[serde(other)]
    Other,
}

pub fn spawn_coinbase_client(state: AppState) {
    tokio::spawn(async move {
        loop {
            let _ = connect_and_stream(&state).await;
            sleep(Duration::from_secs(2)).await;
        }
    });
}

async fn connect_and_stream(state: &AppState) -> anyhow::Result<()> {
    let (mut ws, _resp) = connect_async(&state.config.source_url).await?;
    let sub = serde_json::json!({
        "type": "subscribe",
        "product_ids": ["EIGEN-USD"],
        "channels": ["matches"]
    });
    ws.send(Message::Text(Utf8Bytes::from(sub.to_string())))
        .await?;

    while let Some(frame) = ws.next().await {
        let text = match frame {
            Ok(Message::Text(txt)) => txt.to_owned(),
            Ok(Message::Binary(bin)) => Utf8Bytes::from(String::from_utf8_lossy(&bin).into_owned()),
            Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => continue,
            Ok(Message::Close(_)) => return Ok(()),
            Err(_) => return Ok(()),
            _ => continue,
        };

        if let Ok(parsed) = serde_json::from_str::<CbMsg>(&text) {
            match parsed {
                CbMsg::Match {
                    product_id,
                    price,
                    time,
                }
                | CbMsg::LastMatch {
                    product_id,
                    price,
                    time,
                } => {
                    let tick = Tick {
                        product_id: product_id.clone(),
                        price,
                        time,
                    };
                    if let Some(tx) = get_or_create_sender(state, &product_id).await {
                        let _ = tx.send(tick);
                    }
                }
                CbMsg::Other => {}
            }
        }
    }

    Ok(())
}

async fn get_or_create_sender(state: &AppState, key: &str) -> Option<broadcast::Sender<Tick>> {
    {
        let map = state.broadcasters.read().await;
        if let Some(tx) = map.get(key) {
            return Some(tx.clone());
        }
    }
    let mut map = state.broadcasters.write().await;
    if let Some(existing) = map.get(key) {
        return Some(existing.clone());
    }
    let (tx, _rx) = broadcast::channel(1024);
    map.insert(key.to_string(), tx.clone());
    Some(tx)
}
