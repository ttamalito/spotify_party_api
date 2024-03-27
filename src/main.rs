mod controllers;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use controllers::base_controller::{self, hello};
use controllers::base_controller::*;
use controllers::auth_controller::*;

async fn dummy() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(foo)
            .service(post_login) // /login (POST)
            .route("/hey", web::get().to(dummy))
    })
    .workers(1)
    .on_connect(|a, b| {println!("{}", String::from("A new Connection!"))})
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
