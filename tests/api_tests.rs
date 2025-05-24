use actix_web::{test, App, web, http::StatusCode};

// Import the handler function from the main application
// Since we're testing the application externally, we need to re-define the handler here
// or refactor the main app to expose the handlers through a lib.rs file
#[actix_web::test]
async fn test_get_pizza_endpoint() {
    // Create a test app service with the same configuration as your main app
    let app = test::init_service(
        App::new().service(
            web::resource("/pizza").route(web::get().to(|| async {
                actix_web::HttpResponse::Ok().body("Pizza that are available")
            }))
        )
    ).await;

    // Create a test request
    let req = test::TestRequest::get().uri("/pizza").to_request();
    
    // Call the endpoint and get the response
    let resp = test::call_service(&app, req).await;
    
    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Get the response body and check it
    let body = test::read_body(resp).await;
    assert_eq!(body, "Pizza that are available");
}

#[actix_web::test]
async fn test_nonexistent_endpoint() {
    // Create a test app service
    let app = test::init_service(
        App::new().service(
            web::resource("/pizza").route(web::get().to(|| async {
                actix_web::HttpResponse::Ok().body("Pizza that are available")
            }))
        )
    ).await;

    // Create a test request for a route that doesn't exist
    let req = test::TestRequest::get().uri("/nonexistent").to_request();
    
    // Call the endpoint and expect a 404
    let resp = test::call_service(&app, req).await;
    
    // Assert the response status is 404 Not Found
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}