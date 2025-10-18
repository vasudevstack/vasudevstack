use actix_web::{App, HttpServer};
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(
                actix_web::web::scope("/api")
                    .service(routes::health::health_check_resource())
                    .service(routes::auth::login_resource())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
