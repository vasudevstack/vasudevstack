use actix_web::{web, HttpResponse, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status":"ok"}))
}

// Use resource + route explicitly
pub fn health_check_resource() -> impl actix_web::dev::HttpServiceFactory {
    web::resource("/health").route(web::get().to(health_check))
}
