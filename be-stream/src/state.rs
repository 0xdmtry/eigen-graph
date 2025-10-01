#[derive(Debug, Clone)]
pub struct AppState {
    pub ts_db: Option<sqlx::Pool<sqlx::Postgres>>,
}
