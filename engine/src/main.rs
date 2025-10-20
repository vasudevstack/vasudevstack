use actix_web::{App, HttpServer, web};
mod routes;
mod services;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on 0.0.0.0:8080 (allowing all hosts)");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(routes::health::health_check_resource())
                    .service(routes::auth::login_resource())
                    .service(routes::protected::protected_resource())
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
