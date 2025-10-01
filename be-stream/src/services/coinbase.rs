use crate::{models::tick::Tick, state::AppState};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::{
    select,
    sync::{broadcast, mpsc},
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

#[derive(Debug, Clone)]
pub enum Control {
    Subscribe(String),
    Unsubscribe(String),
}
pub fn spawn_coinbase_client(state: AppState, mut control_rx: mpsc::Receiver<Control>) {
    tokio::spawn(async move {
        loop {
            let _ = connect_and_stream(&state, &mut control_rx).await;
            sleep(Duration::from_secs(2)).await;
        }
    });
}

async fn connect_and_stream(
    state: &AppState,
    control_rx: &mut mpsc::Receiver<Control>,
) -> anyhow::Result<()> {
    let (mut ws, _resp) = match connect_async(&state.config.source_url).await {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };

    let initial: Vec<String> = {
        let counts = state.sub_counts.read().await;
        counts
            .iter()
            .filter(|(_, c)| **c > 0)
            .map(|(k, _)| k.clone())
            .collect()
    };
    if !initial.is_empty() {
        let sub = serde_json::json!({
            "type": "subscribe",
            "product_ids": initial,
            "channels": ["matches"]
        });
        let _ = ws
            .send(Message::Text(Utf8Bytes::from(sub.to_string())))
            .await;
    }

    loop {
        select! {
            biased;

            ctrl = control_rx.recv() => {
                match ctrl {
                    Some(Control::Subscribe(sym)) => {
                        let sub = serde_json::json!({
                            "type": "subscribe",
                            "product_ids": [sym],
                            "channels": ["matches"]
                        });
                        let _ = ws.send(Message::Text(Utf8Bytes::from(sub.to_string()))).await;
                    }
                    Some(Control::Unsubscribe(sym)) => {
                        let unsub = serde_json::json!({
                            "type": "unsubscribe",
                            "product_ids": [sym],
                            "channels": ["matches"]
                        });
                        let _ = ws.send(Message::Text(Utf8Bytes::from(unsub.to_string()))).await;
                    }
                    None => return Ok(()),
                }
            }

            frame = ws.next() => {
                let Some(frame) = frame else { return Ok(()); };
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
                        CbMsg::Match { product_id, price, time }
                        | CbMsg::LastMatch { product_id, price, time } => {
                            let tick = Tick { product_id: product_id.clone(), price, time };
                            if let Some(tx) = get_or_create_sender(state, &product_id).await {
                                let _ = tx.send(tick);
                            }
                        }
                        CbMsg::Other => {}
                    }
                }
            }
        }
    }
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
