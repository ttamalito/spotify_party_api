
use actix_web::{web, App, HttpResponse, HttpServer, Responder, put, options};


#[options("/shuffleOn")]
pub async fn options_shuffleOn() -> impl Responder {
    HttpResponse::Ok().finish()
}