use crate::models::ErrorResponse;
use actix_web::HttpResponse;

pub fn build_success_response(json: &str, cache_ttl: u64) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header((
            "Access-Control-Allow-Headers",
            "Origin, X-Requested-With, Content-Type, Accept",
        ))
        .append_header(("Cache-Control", format!("public, max-age={}", cache_ttl)))
        .body(json.to_string())
}

pub fn create_error_response(message: &str, status: u16, transaction_code: &str) -> HttpResponse {
    let error_response = ErrorResponse {
        transaction_code: transaction_code.to_string(),
        status,
        error: message.to_string(),
    };

    HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap())
        .content_type("application/json")
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header((
            "Access-Control-Allow-Headers",
            "Origin, X-Requested-With, Content-Type, Accept",
        ))
        .body(serde_json::to_string(&error_response).unwrap())
}
