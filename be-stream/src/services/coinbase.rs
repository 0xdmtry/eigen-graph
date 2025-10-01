use crate::config::AppConfig;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::time::{Duration, sleep};
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
        trade_id: Option<u64>,
    },
    #[serde(rename = "last_match")]
    LastMatch {
        product_id: String,
        price: String,
        time: String,
        trade_id: Option<u64>,
    },
    #[serde(other)]
    Other,
}

pub fn spawn_coinbase_client(config: AppConfig) {
    tokio::spawn(async move {
        loop {
            let _ = connect_and_stream(&config.source_url).await;
            sleep(Duration::from_secs(2)).await;
        }
    });
}

async fn connect_and_stream(url: &str) -> anyhow::Result<()> {
    let (mut ws, _resp) = connect_async(url).await?;
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
                CbMsg::Match { .. } | CbMsg::LastMatch { .. } => {}
                CbMsg::Other => {}
            }
        }
    }

    Ok(())
}
