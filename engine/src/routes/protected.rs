use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::middleware::extract_claims_from_request;

async fn profile_handler(req: HttpRequest) -> impl Responder {
    // Extract user claims from the request
    match extract_claims_from_request(&req) {
        Ok(claims) => HttpResponse::Ok().json(json!({
            "status": "success",
            "user": {
                "username": claims.username,
                "user_id": claims.sub,
                "token_id": claims.jti,
                "issued_at": claims.iat,
                "expires_at": claims.exp
            }
        })),
        Err(response) => response,
    }
}

async fn dashboard_handler(req: HttpRequest) -> impl Responder {
    match extract_claims_from_request(&req) {
        Ok(claims) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": format!("Welcome to dashboard, {}!", claims.username),
            "data": {
                "user": claims.username,
                "authenticated": true
            }
        })),
        Err(response) => response,
    }
}

pub fn protected_resource() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/protected")
        .route("/profile", web::get().to(profile_handler))
        .route("/dashboard", web::get().to(dashboard_handler))
}
