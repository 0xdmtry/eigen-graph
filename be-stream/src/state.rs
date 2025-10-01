use crate::config::AppConfig;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::{RwLock, broadcast};

use crate::models::tick::Tick;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub ts_db: Option<sqlx::Pool<sqlx::Postgres>>,
    pub broadcasters: Arc<RwLock<HashMap<String, broadcast::Sender<Tick>>>>,
    pub sub_counts: Arc<RwLock<HashMap<String, usize>>>,
    pub control_tx: mpsc::Sender<crate::services::coinbase::Control>,
}
