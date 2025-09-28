use std::sync::Arc;
use std::time::Duration;

use crate::api::subgraph::client::SubgraphClient;
use crate::config::AppConfig;
use crate::stream::db::fetch_current_bucket;
use crate::stream::db::{DepositRow, insert_batch_ts};
use crate::stream::state::StreamState;
use crate::stream::subgraph::{fetch_deposits_since, resolve_token_id};
use chrono::Utc;

const PAGE_SIZE: i32 = 500;
const REFETCH_WINDOW_SECS: i64 = 60 * 10;

pub async fn run(stream_state: Arc<StreamState>) {
    let cfg = AppConfig::from_env();
    let ts_url = match cfg.timescale_database_url {
        Some(u) => u,
        None => {
            eprintln!("[stream] TIMESCALE_DATABASE_URL not set; poller disabled");
            return;
        }
    };
    let ts_pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&ts_url)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[stream] ts pool error: {e:?}");
            return;
        }
    };
    let sg = SubgraphClient::new(cfg.subgraph_url);

    loop {
        let todo = stream_state.tokens_to_poll();
        for (token_key, cur) in todo {
            let (token_id, token_label) = match resolve_token_id(&sg, &token_key).await {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("[stream] resolve_token_id({token_key}) err: {e}");
                    continue;
                }
            };

            let since = std::cmp::max(
                cur.since_hint,
                cur.last_ts.saturating_sub(REFETCH_WINDOW_SECS),
            );

            let deposits = match fetch_deposits_since(&sg, &token_id, since, PAGE_SIZE).await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[stream] fetch deposits {token_key} err: {e}");
                    continue;
                }
            };

            if deposits.is_empty() {
                continue;
            }

            let mut rows: Vec<DepositRow> = Vec::with_capacity(deposits.len());
            let mut max_ts = cur.last_ts;
            let mut max_id = cur.last_id.clone();

            for d in deposits {
                let ts = d.block_timestamp.parse::<i64>().unwrap_or(0);
                let bn = d.block_number.parse::<i64>().unwrap_or(0);
                let tok = d
                    .token
                    .as_ref()
                    .map(|t| (t.id.clone(), t.symbol.clone()))
                    .unwrap_or_else(|| (token_id.clone(), token_label.clone()));

                if ts < cur.last_ts || (ts == cur.last_ts && d.id <= cur.last_id) {
                    continue;
                }

                rows.push(DepositRow {
                    id: d.id.clone(),
                    token_id: tok.0,
                    token_symbol: tok.1,
                    staker: d.staker.id,
                    strategy_id: d.strategy.id,
                    shares: d.shares,
                    block_number: bn,
                    block_timestamp: ts,
                    tx_hash: d.transaction_hash,
                });

                if ts > max_ts || (ts == max_ts && d.id > max_id) {
                    max_ts = ts;
                    max_id = d.id.clone();
                }
            }

            if rows.is_empty() {
                continue;
            }

            match insert_batch_ts(&ts_pool, &rows).await {
                Ok(_n) => {}
                Err(e) => {
                    eprintln!("[stream] insert ts err: {e}");
                }
            }

            for r in &rows {
                let payload = serde_json::json!({
                    "type": "deposit",
                    "token": token_key,
                    "token_id": r.token_id,
                    "token_symbol": r.token_symbol,
                    "staker": r.staker,
                    "strategy_id": r.strategy_id,
                    "shares": r.shares,
                    "block_number": r.block_number,
                    "block_timestamp": r.block_timestamp,
                    "tx_hash": r.tx_hash,
                    "id": r.id,
                });
                let _ = stream_state
                    .publish(&token_key, crate::stream::state::Event(payload.to_string()));
            }

            stream_state.advance_cursor(&token_key, max_ts, &max_id);

            let now = Utc::now().timestamp();
            let bucket_sec = env_i64("DEPOSITS_BUCKET_SEC", 300);
            fn env_i64(name: &str, default: i64) -> i64 {
                std::env::var(name)
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(default)
            }
            let bucket_start = (now / bucket_sec) * bucket_sec;
            let bucket_end = bucket_start + bucket_sec;

            match fetch_current_bucket(&ts_pool, &token_id, bucket_start, bucket_end).await {
                Ok(bp) => {
                    let payload = serde_json::json!({
                        "type": "tick",
                        "token": token_key,
                        "token_id": token_id,
                        "bucket": { "t": bp.t, "count": bp.count, "sum_shares": bp.sum_shares }
                    });
                    let _ = stream_state
                        .publish(&token_key, crate::stream::state::Event(payload.to_string()));
                }
                Err(e) => eprintln!("[stream] tick agg err for {token_key}: {e}"),
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}
