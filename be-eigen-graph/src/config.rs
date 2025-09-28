use reqwest::Url;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub subgraph_url: Url,
    pub database_url: String,
    pub timescale_database_url: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let subgraph = env::var("SUBGRAPH_URL").unwrap_or_default();
        let subgraph_url = Url::from_str(&subgraph).unwrap();
        let database_url = env::var("DATABASE_URL").unwrap_or_default();
        let timescale_database_url = std::env::var("TIMESCALE_DATABASE_URL").ok();

        Self {
            subgraph_url,
            database_url,
            timescale_database_url,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename(file_name).ok();
        Self::from_env()
    }
}
