use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::services::{pam_auth, jwt};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    status: String,
    token: Option<String>,
    message: Option<String>,
    reason: Option<String>,
}

async fn login_handler(payload: web::Json<LoginRequest>) -> impl Responder {
    let username = payload.username.trim();
    let password = payload.password.trim();

    if username.is_empty() {
        return HttpResponse::BadRequest().json(LoginResponse {
            status: "error".to_string(),
            token: None,
            message: None,
            reason: Some("username is required".to_string()),
        });
    }
    if password.is_empty() {
        return HttpResponse::BadRequest().json(LoginResponse {
            status: "error".to_string(),
            token: None,
            message: None,
            reason: Some("password is required".to_string()),
        });
    }

    match pam_auth::authenticate(username, password).await {
        Ok(true) => {
            // Generate JWT token on successful authentication
            match jwt::JWT_SERVICE.generate_token(username.to_string()) {
                Ok(token) => HttpResponse::Ok().json(LoginResponse {
                    status: "success".to_string(),
                    token: Some(token),
                    message: Some("Login successful".to_string()),
                    reason: None,
                }),
                Err(e) => HttpResponse::InternalServerError().json(LoginResponse {
                    status: "error".to_string(),
                    token: None,
                    message: None,
                    reason: Some(format!("Token generation failed: {}", e)),
                }),
            }
        },
        Ok(false) => HttpResponse::Unauthorized().json(LoginResponse {
            status: "failed".to_string(),
            token: None,
            message: None,
            reason: Some("invalid credentials".to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(LoginResponse {
            status: "error".to_string(),
            token: None,
            message: None,
            reason: Some(format!("Authentication error: {}", e)),
        }),
    }
}

// Expose as a resource instead of scope
pub fn login_resource() -> impl actix_web::dev::HttpServiceFactory {
    web::resource("/auth/login").route(web::post().to(login_handler))
}
