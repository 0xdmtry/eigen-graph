use crate::models::operators_snapshot::OperatorsSnapshotData;
use crate::repositories::operators::upsert_operators_snapshot_page;
use sqlx::{Pool, Postgres};

pub async fn persist_operators_snapshot_db(
    pool: &Pool<Postgres>,
    page: &OperatorsSnapshotData,
) -> Result<(), sqlx::Error> {
    upsert_operators_snapshot_page(pool, page).await
}
