use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;


/*
Struct to deserialize the URL Encoded data sent through the form
*/
#[derive(Deserialize)]
struct LoginData {
    email: String,
    password: String
}

#[post("/login")]
async fn post_login(form: web::Form<LoginData>) -> impl Responder {
    println!("Email: {}", form.email);
    println!("Password: {}", form.password);
    HttpResponse::Ok()
    .insert_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
    .body("Thank You for login in")
}