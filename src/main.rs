use actix_web::{get, post, patch, HttpResponse, Responder, HttpServer, App};

#[get("/pizza")]
async fn get_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizza that are available")
}

 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
