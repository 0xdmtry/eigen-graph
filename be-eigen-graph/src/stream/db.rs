use anyhow::Result;
use bigdecimal::BigDecimal;
use sqlx::PgPool;
use sqlx::Row;
use sqlx::{Pool, Postgres, QueryBuilder};
use std::str::FromStr;

pub struct TsDb;

#[derive(Clone, Debug)]
pub struct DepositRow {
    pub id: String,
    pub token_id: String,
    pub token_symbol: String,
    pub staker: String,
    pub strategy_id: String,
    pub shares: String,
    pub block_number: i64,
    pub block_timestamp: i64,
    pub tx_hash: String,
}

#[derive(Clone, Debug)]
pub struct BucketPoint {
    pub t: i64,
    pub count: i64,
    pub sum_shares: String,
}

pub async fn fetch_buckets(
    pool: &Pool<Postgres>,
    token_id: &str,
    since_ts: i64,
    bucket_sec: i64,
) -> Result<Vec<BucketPoint>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
          ((block_timestamp / $2) * $2)::bigint AS bucket,
          COUNT(*)::bigint                      AS cnt,
          COALESCE(SUM(shares), 0)::numeric     AS sum_shares
        FROM deposits_raw
        WHERE token_id = $1 AND block_timestamp >= $3
        GROUP BY bucket
        ORDER BY bucket
        "#,
    )
    .bind(token_id)
    .bind(bucket_sec)
    .bind(since_ts)
    .fetch_all(pool)
    .await?;

    let out = rows
        .into_iter()
        .map(|r| {
            let bucket: i64 = r.get("bucket");
            let cnt: i64 = r.get("cnt");
            let sum: BigDecimal = r.get("sum_shares");
            BucketPoint {
                t: bucket,
                count: cnt,
                sum_shares: sum.to_string(),
            }
        })
        .collect();

    Ok(out)
}

pub async fn fetch_current_bucket(
    pool: &Pool<Postgres>,
    token_id: &str,
    bucket_start: i64,
    bucket_end: i64,
) -> Result<BucketPoint, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
          COUNT(*)::bigint                  AS cnt,
          COALESCE(SUM(shares), 0)::numeric AS sum_shares
        FROM deposits_raw
        WHERE token_id = $1
          AND block_timestamp >= $2
          AND block_timestamp <  $3
        "#,
    )
    .bind(token_id)
    .bind(bucket_start)
    .bind(bucket_end)
    .fetch_one(pool)
    .await?;

    let cnt: i64 = row.get("cnt");
    let sum: BigDecimal = row.get("sum_shares");

    Ok(BucketPoint {
        t: bucket_start,
        count: cnt,
        sum_shares: sum.to_string(),
    })
}

pub async fn insert_batch_ts(
    pool: &Pool<Postgres>,
    rows: &[DepositRow],
) -> Result<u64, sqlx::Error> {
    if rows.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new(
        "INSERT INTO deposits_raw \
         (id, token_id, token_symbol, staker, strategy_id, shares, block_number, block_timestamp, tx_hash) ",
    );
    qb.push_values(rows, |mut b, r| {
        b.push_bind(&r.id)
            .push_bind(&r.token_id)
            .push_bind(&r.token_symbol)
            .push_bind(&r.staker)
            .push_bind(&r.strategy_id)
            .push_bind(
                bigdecimal::BigDecimal::from_str(&r.shares).expect("invalid decimal in shares"),
            )
            .push_bind(r.block_number)
            .push_bind(r.block_timestamp)
            .push_bind(&r.tx_hash);
    });

    qb.push(" ON CONFLICT (tx_hash, block_timestamp) DO NOTHING");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

pub async fn load_cursor(
    pool: &Pool<Postgres>,
    token_id: &str,
) -> Result<Option<(i64, String)>, sqlx::Error> {
    let row = sqlx::query(r#"SELECT last_ts, last_id FROM stream_cursors WHERE token_id = $1"#)
        .bind(token_id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| (r.get::<i64, _>("last_ts"), r.get::<String, _>("last_id"))))
}

pub async fn upsert_cursor(
    pool: &Pool<Postgres>,
    token_id: &str,
    last_ts: i64,
    last_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO stream_cursors (token_id, last_ts, last_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (token_id) DO UPDATE
        SET last_ts = EXCLUDED.last_ts,
            last_id = EXCLUDED.last_id,
            updated_at = now()
        "#,
    )
    .bind(token_id)
    .bind(last_ts)
    .bind(last_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn count_rows_for_token(
    pool: &Pool<Postgres>,
    token_id: &str,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(r#"SELECT COUNT(*)::BIGINT AS c FROM deposits_raw WHERE token_id = $1"#)
        .bind(token_id)
        .fetch_one(pool)
        .await?;

    Ok(row.get::<i64, _>("c"))
}

pub async fn total_shares_text(pool: &PgPool, token_id: &str) -> Result<i128> {
    let row: (Option<String>,) = sqlx::query_as(
        r#"
        SELECT CAST(COALESCE(SUM(shares), 0) AS TEXT)
        FROM deposits_raw
        WHERE token_symbol = $1
        "#,
    )
    .bind(token_id)
    .fetch_one(pool)
    .await?;

    let s = row.0.unwrap_or_else(|| "0".to_string());
    let total_i128 = s.parse::<i128>().unwrap_or(0);
    Ok(total_i128)
}

impl TsDb {
    pub fn new_unused_stub() -> Self {
        Self
    }
}
