use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use crate::services::pam_auth;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

async fn login_handler(payload: web::Json<LoginRequest>) -> impl Responder {
    let username = payload.username.trim();
    let password = payload.password.trim();

    if username.is_empty() {
        return HttpResponse::BadRequest()
            .json(json!({"status":"error","reason":"username is required"}));
    }
    if password.is_empty() {
        return HttpResponse::BadRequest()
            .json(json!({"status":"error","reason":"password is required"}));
    }

    match pam_auth::authenticate(username, password).await {
        Ok(true) => HttpResponse::Ok().json(json!({"status":"success"})),
        Ok(false) => HttpResponse::Unauthorized().json(json!({"status":"failed","reason":"invalid credentials"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status":"error","reason":format!("{}",e)})),
    }
}

// Expose as a resource instead of scope
pub fn login_resource() -> impl actix_web::dev::HttpServiceFactory {
    web::resource("/auth/login").route(web::post().to(login_handler))
}
