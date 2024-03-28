mod controllers;
mod database_connection;


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use controllers::base_controller::{self, hello};
use controllers::base_controller::*;
use controllers::auth_controller::*;

use database_connection::connect_to_db;

async fn dummy() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let res = connect_to_db("puto").await.expect("Verga!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(foo)
            .service(post_login) // /login (POST)
            .service(post_signup) // /signup (POST)
            .route("/hey", web::get().to(dummy))
    })
    .workers(1)
    .on_connect(|a, b| {println!("{}", String::from("A new Connection!"))})
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
