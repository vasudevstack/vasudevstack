use actix_web::{HttpRequest, HttpResponse};
use serde_json::json;
use crate::services::jwt::{JwtService, JWT_SERVICE, Claims};

// Simple authentication extractor function
pub fn extract_claims_from_request(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let auth_header = req.headers().get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(json!({
                "status": "error",
                "reason": "Authorization header required"
            }))
        })?;

    let token = JwtService::extract_token_from_header(auth_header)
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(json!({
                "status": "error",
                "reason": "Invalid authorization header format"
            }))
        })?;

    JWT_SERVICE.validate_token(&token)
        .map_err(|_| {
            HttpResponse::Unauthorized().json(json!({
                "status": "error",
                "reason": "Invalid or expired token"
            }))
        })
}
