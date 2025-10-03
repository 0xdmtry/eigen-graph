use reqwest::Url;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub subgraph_url: Url,
    pub database_url: String,
    pub redis_url: String,
    pub redis_ttl_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let subgraph = env::var("SUBGRAPH_URL").unwrap_or_default();
        let subgraph_url = Url::from_str(&subgraph).unwrap();
        let database_url = env::var("DATABASE_URL").unwrap_or_default();
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis-eigen-graph:6379".to_string());
        let redis_ttl_seconds = env::var("REDIS_TTL_SECONDS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(60);

        Self {
            subgraph_url,
            database_url,
            redis_url,
            redis_ttl_seconds,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename(file_name).ok();
        Self::from_env()
    }
}
