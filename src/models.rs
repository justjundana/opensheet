use governor::clock::DefaultClock;
use governor::state::keyed::DefaultKeyedStateStore;
use governor::RateLimiter;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// Type alias for keyed rate limiter based on IP addresses.
pub type KeyedRateLimiter = RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>;

/// Represents a single cache entry, including its timestamp.
pub struct CacheEntry {
    pub data: String,
    pub timestamp: Instant,
}

/// Shared application state for Actix handlers.
pub struct AppState {
    pub cache: Mutex<HashMap<String, CacheEntry>>,
    pub client: Client,
    pub google_api_key: String,
    pub rate_limiter: Arc<KeyedRateLimiter>,
    pub cache_ttl: std::time::Duration,
}

/// Wrapper for API responses.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub transaction_code: String,
    pub status: u16,
    pub data: T,
}

/// Wrapper for error responses.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub transaction_code: String,
    pub status: u16,
    pub error: String,
}

/// Response format from Google Sheets API.
#[derive(Deserialize, Debug)]
pub struct SheetsApiResponse {
    pub values: Option<Vec<Vec<String>>>,
    pub error: Option<ErrorDetails>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorDetails {
    pub message: String,
    pub status: Option<String>,
}

/// Metadata structure returned by the Sheets API when querying spreadsheet info.
#[derive(Deserialize, Debug)]
pub struct SheetMetadata {
    pub sheets: Vec<Sheet>,
    pub error: Option<ErrorDetails>,
}

#[derive(Deserialize, Debug)]
pub struct Sheet {
    pub properties: SheetProperties,
}

#[derive(Deserialize, Debug)]
pub struct SheetProperties {
    pub title: String,
}
