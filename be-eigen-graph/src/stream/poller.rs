use std::sync::Arc;

use crate::api::subgraph::client::SubgraphClient;
use crate::config::AppConfig;
use crate::stream::db::insert_batch_ts;
use crate::stream::db::{
    DepositRow, count_rows_for_token, fetch_current_bucket, load_cursor, upsert_cursor,
};
use crate::stream::state::StreamState;
use crate::stream::subgraph::fetch_deposits_since_desc;
use crate::stream::subgraph::{fetch_deposits_since, resolve_token_id};

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

    let page_size = env_i32("DEPOSITS_PAGE_SIZE", 500);
    let bootstrap_days = env_i64("DEPOSITS_BOOTSTRAP_LOOKBACK_DAYS", 365);
    let bootstrap_max_pages = env_i32("DEPOSITS_BOOTSTRAP_MAX_PAGES", 50);
    const REFETCH_WINDOW_SECS: i64 = 60 * 10;

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

            let db_cursor = match load_cursor(&ts_pool, &token_id).await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[stream] load_cursor({token_key}) err: {e}");
                    None
                }
            };
            if let Some((ts, id)) = db_cursor.clone() {
                stream_state.seed_cursor_if_empty(&token_key, ts, &id);
            }

            match count_rows_for_token(&ts_pool, &token_id).await {
                Ok(0) => {
                    let since_ts = chrono::Utc::now().timestamp() - bootstrap_days * 86_400;
                    eprintln!(
                        "[stream] bootstrap {token_key} since={since_ts} days={bootstrap_days}"
                    );
                    match bootstrap_backfill(
                        &ts_pool,
                        &sg,
                        &token_id,
                        &token_key,
                        since_ts,
                        page_size,
                        bootstrap_max_pages,
                    )
                    .await
                    {
                        Ok((ts, id, n)) => {
                            eprintln!("[stream] bootstrap {token_key} inserted={n} max_ts={ts}");
                            if ts > 0 {
                                stream_state.advance_cursor(&token_key, ts, &id);
                            }
                        }
                        Err(e) => eprintln!("[stream] bootstrap err for {token_key}: {e}"),
                    }
                }
                Ok(_) => {  }
                Err(e) => eprintln!("[stream] count_rows_for_token({token_key}) err: {e}"),
            }

            let (last_ts, last_id) = if cur.last_ts > 0 || !cur.last_id.is_empty() {
                (cur.last_ts, cur.last_id.clone())
            } else if let Some((ts, id)) = db_cursor.clone() {
                (ts, id)
            } else {
                (0, String::new())
            };

            let since = std::cmp::max(cur.since_hint, last_ts.saturating_sub(REFETCH_WINDOW_SECS));

            let deposits = match fetch_deposits_since(&sg, &token_id, since, page_size).await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[stream] fetch deposits {token_key} err: {e}");
                    emit_tick(&ts_pool, &stream_state, &token_key, &token_id).await;
                    continue;
                }
            };

            emit_tick(&ts_pool, &stream_state, &token_key, &token_id).await;

            if !deposits.is_empty() {
                let mut rows: Vec<DepositRow> = Vec::with_capacity(deposits.len());
                let mut max_ts = last_ts;
                let mut max_id = last_id.clone();

                for d in deposits {
                    let ts = d.block_timestamp.parse::<i64>().unwrap_or(0);
                    let bn = d.block_number.parse::<i64>().unwrap_or(0);
                    let tok = d
                        .token
                        .as_ref()
                        .map(|t| (t.id.clone(), t.symbol.clone()))
                        .unwrap_or_else(|| (token_id.clone(), token_label.clone()));

                    if ts < last_ts || (ts == last_ts && d.id <= last_id) {
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

                if !rows.is_empty() {
                    match insert_batch_ts(&ts_pool, &rows).await {
                        Ok(_n) => { /* inserted */ }
                        Err(e) => eprintln!("[stream] insert ts err: {e}"),
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

                    // Persist + advance in-memory cursor
                    if max_ts > last_ts || (max_ts == last_ts && max_id > last_id) {
                        if let Err(e) = upsert_cursor(&ts_pool, &token_id, max_ts, &max_id).await {
                            eprintln!("[stream] upsert_cursor err for {token_key}: {e}");
                        }
                        stream_state.advance_cursor(&token_key, max_ts, &max_id);
                    }
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

async fn emit_tick(
    ts_pool: &sqlx::Pool<sqlx::Postgres>,
    stream_state: &StreamState,
    token_key: &str,
    token_id: &str,
) {
    let now = chrono::Utc::now().timestamp();
    let bucket_sec = std::env::var("DEPOSITS_BUCKET_SEC")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300i64);

    let bucket_start = (now / bucket_sec) * bucket_sec;
    let bucket_end = bucket_start + bucket_sec;

    match fetch_current_bucket(ts_pool, token_id, bucket_start, bucket_end).await {
        Ok(bp) => {
            let payload = serde_json::json!({
                "type": "tick",
                "token": token_key,
                "token_id": token_id,
                "bucket": { "t": bp.t, "count": bp.count, "sum_shares": bp.sum_shares }
            });
            let _ =
                stream_state.publish(token_key, crate::stream::state::Event(payload.to_string()));
        }
        Err(e) => eprintln!("[stream] tick agg err for {token_key}: {e}"),
    }
}

fn env_i64(name: &str, default: i64) -> i64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
fn env_i32(name: &str, default: i32) -> i32 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

async fn bootstrap_backfill(
    ts_pool: &sqlx::Pool<sqlx::Postgres>,
    sg: &SubgraphClient,
    token_id: &str,
    token_key: &str,
    since_ts: i64,
    page_size: i32,
    max_pages: i32,
) -> anyhow::Result<(i64, String, usize)> {
    let mut skip = 0;
    let mut total_inserted = 0usize;
    let mut max_ts = 0i64;
    let mut max_id = String::new();

    for _ in 0..max_pages {
        let page = fetch_deposits_since_desc(sg, token_id, since_ts, page_size, skip).await?;
        if page.is_empty() {
            break;
        }

        let mut rows = Vec::with_capacity(page.len());
        for d in page.iter() {
            let ts = d.block_timestamp.parse::<i64>().unwrap_or(0);
            let bn = d.block_number.parse::<i64>().unwrap_or(0);
            let tok = d
                .token
                .as_ref()
                .map(|t| (t.id.clone(), t.symbol.clone()))
                .unwrap_or_else(|| (token_id.to_string(), token_key.to_string()));

            rows.push(DepositRow {
                id: d.id.clone(),
                token_id: tok.0,
                token_symbol: tok.1,
                staker: d.staker.id.clone(),
                strategy_id: d.strategy.id.clone(),
                shares: d.shares.clone(),
                block_number: bn,
                block_timestamp: ts,
                tx_hash: d.transaction_hash.clone(),
            });

            if ts > max_ts || (ts == max_ts && d.id > max_id) {
                max_ts = ts;
                max_id = d.id.clone();
            }
        }

        let inserted = insert_batch_ts(ts_pool, &rows).await? as usize;
        total_inserted += inserted;

        if page.len() < page_size as usize {
            break;
        }
        skip += page_size;
    }

    if max_ts > 0 {
        upsert_cursor(ts_pool, token_id, max_ts, &max_id).await?;
    }
    Ok((max_ts, max_id, total_inserted))
}
