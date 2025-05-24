use actix_web::{get, post, patch, HttpResponse, Responder, web};


#[get("/pizza")]
pub async fn get_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizza that are available")
}

// Configure the application with all route handlers
pub fn configure_app(config: &mut web::ServiceConfig) {
    config.service(get_pizza);
}

// Function that creates an App with all routes configured
/* 
pub fn create_app() -> impl actix_web::ServiceFactory {
    App::new().configure(configure_app)
}
    */

use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, Error, App};

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = ()
    >
> {
    App::new().configure(configure_app)
}