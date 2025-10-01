use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub ts_db: Option<sqlx::Pool<sqlx::Postgres>>,
}
