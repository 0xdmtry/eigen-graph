use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, broadcast};

use crate::config::AppConfig;
use crate::models::tick::Tick;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub ts_db: Option<sqlx::Pool<sqlx::Postgres>>,
    pub broadcasters: Arc<RwLock<HashMap<String, broadcast::Sender<Tick>>>>,
}
