use actix_web::{HttpServer};
use rust_backend::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        create_app()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
