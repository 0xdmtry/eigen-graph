use crate::models::tick::Tick;
use sqlx::{Pool, Postgres};

pub async fn insert_tick(pool: &Pool<Postgres>, t: &Tick) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO ticks (product_id, time, price) VALUES ($1, $2::timestamptz, $3::numeric)",
    )
    .bind(&t.product_id)
    .bind(&t.time)
    .bind(&t.price)
    .execute(pool)
    .await
    .map(|_| ())
}
