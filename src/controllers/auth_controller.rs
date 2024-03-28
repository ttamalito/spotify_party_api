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


/*
Struct to deserialize the Form of signup
*/
#[derive(Deserialize)]
struct SignupData {
    name: String,
    email: String,
    password: String
}

#[post("/signup")]
async fn post_signup(form: web::Form<SignupData>) -> impl Responder {
    let email = &form.email;
    let name = &form.name;
    let password = &form.password;

    // here we would need to hash the password

    // send the response
    HttpResponse::Ok()
    .insert_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
    .body("Thank you for signin up")
}
