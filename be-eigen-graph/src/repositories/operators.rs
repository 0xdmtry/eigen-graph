use crate::models::operators_snapshot::OperatorsSnapshotData;
use sqlx::{PgPool, Postgres, Transaction};

pub async fn upsert_operators_snapshot_page(
    pool: &PgPool,
    page: &OperatorsSnapshotData,
) -> Result<(), sqlx::Error> {
    let mut tx: Transaction<Postgres> = pool.begin().await?;

    for op in &page.operators {
        let last_slash_at = op
            .slashings
            .first()
            .and_then(|s| s.block_timestamp.parse::<i64>().ok());

        sqlx::query(
            r#"
                INSERT INTO operators_snapshot
                        (operator_id, avs_count, strategy_count, slashing_count, last_slash_at, last_update_block_ts)
                VALUES ($1,$2,$3,$4,$5,$6)
                ON CONFLICT (operator_id) DO UPDATE
                    SET avs_count = EXCLUDED.avs_count,
                        strategy_count = EXCLUDED.strategy_count,
                        slashing_count = EXCLUDED.slashing_count,
                        last_slash_at = EXCLUDED.last_slash_at,
                        last_update_block_ts = EXCLUDED.last_update_block_ts
                WHERE operators_snapshot.last_update_block_ts < EXCLUDED.last_update_block_ts
                "#,
        )
            .bind(&op.id)
            .bind(op.avs_count)
            .bind(op.strategy_count)
            .bind(op.slashing_count)
            .bind(last_slash_at)
            .bind(op.last_update_block_timestamp.parse::<i64>().unwrap_or(0))
            .execute(tx.as_mut())
            .await?;

        sqlx::query("DELETE FROM operator_strategy WHERE operator_id = $1")
            .bind(&op.id)
            .execute(tx.as_mut())
            .await?;

        for link in &op.strategies {
            let Some(token) = link.strategy.token.as_ref() else {
                continue;
            };

            sqlx::query(
                r#"
                    INSERT INTO operator_strategy
                        (operator_id, strategy_id, token_id, token_symbol, token_decimals, total_shares, exchange_rate)
                    VALUES ($1,$2,$3,$4,$5,$6,$7)
                "#
            )
                .bind(&op.id)
                .bind(&link.strategy.id)
                .bind(&token.id)
                .bind(&token.symbol)
                .bind(token.decimals)
                .bind(&link.total_shares)
                .bind(&link.strategy.exchange_rate)
                .execute(tx.as_mut()).await?;
        }
    }

    tx.commit().await
}
