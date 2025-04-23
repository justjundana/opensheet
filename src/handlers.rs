use crate::models::{ApiResponse, AppState, SheetMetadata, SheetQuery, SheetsApiResponse};
use crate::services::{get_sheet_name, process_sheet_data};
use crate::utils::{build_success_response, create_error_response};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use log::{error, info};
use std::sync::Arc;
use uuid::Uuid;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/health", web::get().to(health_check))
        .route("/{id}/{sheet_name}", web::get().to(get_sheet));
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Found()
        .append_header((
            "Location",
            "https://github.com/justjundana/opensheet#readme",
        ))
        .finish())
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn get_sheet(
    path: web::Path<(String, String)>,
    query: web::Query<SheetQuery>,
    data: web::Data<Arc<AppState>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (id, sheet_name) = path.into_inner();
    let range_param = query.range.clone();

    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let transaction_code = Uuid::new_v4().to_string();

    if data.rate_limiter.check_key(&ip).is_err() {
        info!("Rate limit exceeded for IP: {}", ip);
        return Ok(create_error_response(
            "Rate limit exceeded. Try again later.",
            429,
            &transaction_code,
        ));
    }

    let cache_key = format!("{}/{}", id, sheet_name);

    if let Some(entry) = data.cache.lock().await.get(&cache_key) {
        if entry.timestamp.elapsed() < data.cache_ttl {
            info!("Serving from cache: {}", cache_key);
            return Ok(build_success_response(
                &entry.data,
                data.cache_ttl.as_secs(),
            ));
        }
    }

    info!("Cache miss for: {} (IP: {})", cache_key, ip);

    let sheet = match get_sheet_name(&id, &sheet_name, &data).await {
        Ok(sheet) => sheet,
        Err((msg, status)) => {
            return Ok(create_error_response(&msg, status, &transaction_code));
        }
    };

    // Smart range: custom if provided, else default
    let range = if let Some(custom_range) = range_param {
        format!("{}!{}", urlencoding::encode(&sheet), custom_range)
    } else {
        format!("{}!A1:ZZ", urlencoding::encode(&sheet))
    };

    let api_url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        id, range, data.google_api_key
    );

    let response = match data.client.get(&api_url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("API request failed: {}", e);
            return Ok(create_error_response(
                &format!("API request failed: {}", e),
                500,
                &transaction_code,
            ));
        }
    };

    let result: SheetsApiResponse = match response.json().await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to parse API response: {}", e);
            return Ok(create_error_response(
                &format!("Failed to parse API response: {}", e),
                500,
                &transaction_code,
            ));
        }
    };

    if let Some(error) = result.error {
        let status = error
            .status
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(400);
        return Ok(create_error_response(
            &error.message,
            status,
            &transaction_code,
        ));
    }

    let values = result.values.unwrap_or_default();

    let rows = process_sheet_data(&values);
    let formatted_response = ApiResponse {
        transaction_code: transaction_code.clone(),
        status: 200,
        data: rows,
    };

    let json = match serde_json::to_string(&formatted_response) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize response: {}", e);
            return Ok(create_error_response(
                &format!("Failed to serialize response: {}", e),
                500,
                &transaction_code,
            ));
        }
    };

    data.cache.lock().await.insert(
        cache_key,
        crate::models::CacheEntry {
            data: json.clone(),
            timestamp: std::time::Instant::now(),
        },
    );

    Ok(build_success_response(&json, data.cache_ttl.as_secs()))
}
