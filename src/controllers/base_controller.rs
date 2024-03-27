use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
// controller for the main page
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!- Using Rust!- And cargo watch")
}

#[get("/response")]
async fn foo(req: HttpRequest) -> impl Responder {
    // print the request
    println!("{:?}", req);
    HttpResponse::Ok().body("I saw your request")
}