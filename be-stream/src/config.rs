use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub source_url: String,
    pub timescale_database_url: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let source_url = env::var("SOURCE_URL").unwrap_or_default();
        let timescale_database_url = std::env::var("TIMESCALE_DATABASE_URL").ok();

        Self {
            source_url,
            timescale_database_url,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename(file_name).ok();
        Self::from_env()
    }
}
