use crate::{models::tick::Tick, repositories::ticks::insert_tick};
use sqlx::{Pool, Postgres};
use tokio::sync::mpsc;

pub fn spawn_writer(ts_pool: Option<Pool<Postgres>>, mut rx: mpsc::Receiver<Tick>) {
    if let Some(pool) = ts_pool {
        tokio::spawn(async move {
            while let Some(tick) = rx.recv().await {
                if let Err(e) = insert_tick(&pool, &tick).await {
                    eprintln!("[writer] insert failed for {tick:?}: {e}");
                }
            }
        });
    } else {
        tokio::spawn(async move { while rx.recv().await.is_some() {} });
    }
}
