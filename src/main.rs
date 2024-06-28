// declare all the modules
pub mod controllers;
pub mod database_connection;
pub mod application_data;
pub mod utils;
pub mod models;
pub mod constants;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use controllers::base_controller::{hello};
use controllers::base_controller::*;
use controllers::auth_controller::*;
use controllers::party_controller::*;
use controllers::options_controller::*;
use controllers::playback_controller::*;
use application_data::ApplicationData;

use database_connection::{connect_to_db};



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
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT"))) // add a middleware to add the default header
            .app_data(data.clone())
            .service(hello)
            .service(foo)
            
            .service(post_login) // /login (POST)
            .service(post_signup) // /signup (POST)
            .service(start_party) // /party (GET)
            .service(start_party_two)
            .service(request_token)
            .service(pause_playback) // pause playback
            .service(resume_playback) // resume playback
            .service(play_next) // paly next song
            .service(play_previous) // play previous
            .service(modify_volume) // modify the volume
            .service(join_party) // GET route to join a party
            .service(acceptIntoParty) // accept a user into a party
            .service(user_has_party) // check if the user has a party
            .service(turn_on_shuffle)
            .service(options_shuffleOn) // options request from the browser
            .service(turn_off_shuffle)
            .service(options_shuffleOff)
            .route("/hey", web::get().to(dummy))
    })
    .workers(4)
    .on_connect(|_a, _b| {println!("{}", String::from("A new Connection!"))})
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
