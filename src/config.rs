use anyhow::{Context, Result};

pub struct Config {
    pub google_api_key: String,
    pub port: u16,
    pub host: String,
    pub cache_ttl_seconds: u64,
    pub request_timeout_seconds: u64,
    pub rate_limit_per_minute: u32,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// Will return an error if any required variable is missing or invalid.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            google_api_key: std::env::var("GOOGLE_API_KEY")
                .context("GOOGLE_API_KEY must be set")?,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("PORT must be a valid number")?,
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            cache_ttl_seconds: std::env::var("CACHE_TTL_SECONDS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .context("CACHE_TTL_SECONDS must be a valid number")?,
            request_timeout_seconds: std::env::var("REQUEST_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("REQUEST_TIMEOUT_SECONDS must be a valid number")?,
            rate_limit_per_minute: std::env::var("RATE_LIMIT_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .context("RATE_LIMIT_PER_MINUTE must be a valid number")?,
        })
    }
}
