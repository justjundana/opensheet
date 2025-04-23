mod config;
mod handlers;
mod models;
mod services;
mod utils;

use actix_web::{web, App, HttpServer};
use config::Config;
use env_logger::Env;
use governor::{Quota, RateLimiter};
use log::info;
use models::AppState;
use reqwest::Client;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load configuration from .env
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize HTTP client
    let client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout_seconds))
        .build()
        .expect("Failed to create HTTP client");

    // Setup rate limiter (per IP address)
    let rate_limiter = Arc::new(RateLimiter::keyed(Quota::per_minute(
        NonZeroU32::new(config.rate_limit_per_minute.max(1)).unwrap(),
    )));

    let cache_ttl = Duration::from_secs(config.cache_ttl_seconds);

    // Shared state across handlers
    let app_state = Arc::new(AppState {
        cache: Mutex::new(HashMap::new()),
        client,
        google_api_key: config.google_api_key,
        rate_limiter,
        cache_ttl,
    });

    let bind_address = format!("{}:{}", config.host, config.port);
    info!("Starting server at http://{}", bind_address);

    // Start Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(handlers::init_routes)
    })
    .workers(4)
    .keep_alive(Duration::from_secs(60))
    .client_request_timeout(Duration::from_secs(60))
    .bind(&bind_address)?
    .run()
    .await
}
