use crate::models::operators_aggr::{PageMeta, UniformOperator, UniformPage, UniformPosition};
use sqlx::Row;
use sqlx::{Pool, Postgres};
use std::collections::BTreeMap;

pub async fn from_db_adapt(
    pool: &Pool<Postgres>,
    first: i32,
    skip: i32,
) -> Result<UniformPage, sqlx::Error> {
    let ops_rows = sqlx::query(
        r#"
    SELECT operator_id, avs_count, strategy_count, slashing_count,
           last_slash_at, last_update_block_ts
    FROM operators_snapshot
    ORDER BY last_update_block_ts DESC, operator_id ASC
    LIMIT $1 OFFSET $2
    "#,
    )
    .bind(first as i64)
    .bind(skip as i64)
    .fetch_all(pool)
    .await?;

    if ops_rows.is_empty() {
        return Ok(UniformPage {
            operators: vec![],
            page_meta: PageMeta { first, skip },
        });
    }

    let op_ids: Vec<String> = ops_rows
        .iter()
        .map(|r| r.get::<String, _>("operator_id"))
        .collect();
    let op_id_refs: Vec<&str> = op_ids.iter().map(AsRef::as_ref).collect();

    let pos_rows = sqlx::query(
        r#"
    SELECT operator_id, strategy_id, token_id, token_symbol, token_decimals,
           total_shares, exchange_rate
    FROM operator_strategy
    WHERE operator_id = ANY($1)
    "#,
    )
    .bind(&op_id_refs[..])
    .fetch_all(pool)
    .await?;

    let mut pos_map: BTreeMap<String, Vec<UniformPosition>> = BTreeMap::new();
    for r in pos_rows {
        let operator_id: String = r.get("operator_id");
        pos_map
            .entry(operator_id)
            .or_default()
            .push(UniformPosition {
                strategy_id: r.get("strategy_id"),
                token_id: r.get("token_id"),
                token_symbol: r.get("token_symbol"),
                token_decimals: r.get::<i32, _>("token_decimals"),
                total_shares: r.get("total_shares"),
                exchange_rate: r.get("exchange_rate"),
            });
    }

    let operators = ops_rows
        .into_iter()
        .map(|r| {
            let operator_id: String = r.get("operator_id");
            UniformOperator {
                operator_id: operator_id.clone(),
                avs_count: r.get::<i32, _>("avs_count"),
                strategy_count: r.get::<i32, _>("strategy_count"),
                slashing_count: r.get::<i32, _>("slashing_count"),
                last_slash_at: r.try_get::<i64, _>("last_slash_at").ok(),
                last_update_block_ts: r.get::<i64, _>("last_update_block_ts"),
                positions: pos_map.remove(&operator_id).unwrap_or_default(),
            }
        })
        .collect();

    Ok(UniformPage {
        operators,
        page_meta: PageMeta { first, skip },
    })
}
