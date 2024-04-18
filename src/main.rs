// declare all the modules
pub mod controllers;
pub mod database_connection;
pub mod application_data;
pub mod utils;
pub mod models;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use controllers::base_controller::{self, hello};
use controllers::base_controller::*;
use controllers::auth_controller::*;
use controllers::party_controller::*;
use application_data::ApplicationData;

use database_connection::{connect_to_db, DB};



async fn dummy() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_box = connect_to_db("puto").await.expect("Verga!");
    let my = ApplicationData::new(db_box);
    let data = Data::new(my);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "http://localhost:3000"))) // add a middleware to add the default header
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Credentials", "true")))
            .app_data(data.clone())
            .service(hello)
            .service(foo)
            
            .service(post_login) // /login (POST)
            .service(post_signup) // /signup (POST)
            .service(start_party) // /party (GET)
            .service(start_party_two)
            .service(request_token)
            .service(pause_playback) // pause playback
            .route("/hey", web::get().to(dummy))
    })
    .workers(4)
    .on_connect(|a, b| {println!("{}", String::from("A new Connection!"))})
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
