use reqwest::Url;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub subgraph_url: Url,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let subgraph = env::var("SUBGRAPH_URL").unwrap_or_default();
        let subgraph_url = Url::from_str(&subgraph).unwrap();

        Self { subgraph_url }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename(file_name).ok();
        Self::from_env()
    }
}
