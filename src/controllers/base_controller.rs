use actix_web::{get, post, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder};
// controller for the main page
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!- Using Rust!- And cargo watch")
}

#[get("/response")]
async fn foo(req: HttpRequest) -> impl Responder {
    // print the request
    println!("The path: {:?}", req.path());
    println!("The Req method: {:?}", req.method());
    println!("The HTTP version: {:?}", req.version());
    println!("The headers: {:?}", req.headers());
    println!("The app Config: {:?}", req.app_config());
   // println!("The app data: {:?}", req.app_data::<Data<T>>());
    HttpResponse::Ok().body("Te amo")
}