use anyhow::{Context, Result};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub host: String,
    pub log_level: String,
    pub cors_origins: Vec<String>,
    pub max_page_size: usize,
    pub enable_compression: bool,
    pub enable_openapi: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "vehicles.db".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .context("Invalid PORT value")?;

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "*".to_string())
            .split(',')
            .map(String::from)
            .collect();

        let max_page_size = std::env::var("MAX_PAGE_SIZE")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .context("Invalid MAX_PAGE_SIZE value")?;

        let enable_compression = std::env::var("ENABLE_COMPRESSION")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);

        let enable_openapi = std::env::var("ENABLE_OPENAPI")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);

        Ok(Self {
            database_url,
            port,
            host,
            log_level,
            cors_origins,
            max_page_size,
            enable_compression,
            enable_openapi,
        })
    }
}
