use sqlx::{Pool, Postgres, QueryBuilder};

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
            .push_bind(&r.shares)
            .push_bind(r.block_number)
            .push_bind(r.block_timestamp)
            .push_bind(&r.tx_hash);
    });

    qb.push(" ON CONFLICT ON CONSTRAINT ux_deposits_tx_block DO NOTHING");
    let res = qb.build().execute(pool).await?;
    Ok(res.rows_affected())
}

impl TsDb {
    pub fn new_unused_stub() -> Self {
        Self
    }
}
