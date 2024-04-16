use actix_web::{get, http::{header::HeaderValue, StatusCode}, post, web::{self, Json, Redirect}, HttpRequest, HttpResponse, Responder};
use jwt::Store;
use crate::utils::cookie_parser::parse_cookies;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::{collections::BTreeMap, time::Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::check_login::check_login;
use crate::utils::{response::JsonResponse};
use awc::{Connector, Client};
use openssl::ssl::{SslConnector, SslMethod};
#[get("/puto")]
async fn start_party(req: HttpRequest) -> impl Responder {
    let header_map = req.headers();
    println!("{:?}", header_map);
    let cookie_str = "Cookie";
    let _log_token = header_map.get(cookie_str);
    println!("{}", String::from("Que putas esta pasando!?"));
    // check if there is a token
    /*
        if log_token.is_some() {
        println!("{}", String::from("We are about to redirect"));
        // there is no cookie
        // Redirect to login
        //return Redirect::to("/login").permanent();
        return HttpResponse::Ok().status(StatusCode::from_u16(307).unwrap()).insert_header(("Location", "http://localhost:3000/login")).finish();
    } */
    

    HttpResponse::Ok().body("You have started a party")
} // end of start_party

#[get("/createParty")]
async fn start_party_two(req: HttpRequest) -> impl Responder {
    let header_map = req.headers();
    //println!("{:?}", header_map);
    if !check_login(header_map) {
        // not logged in
        let not_logged_in_response = JsonResponse::new(false, true, String::from("http://localhost:3000/login"));
        return HttpResponse::Ok().json(not_logged_in_response);
    }

    HttpResponse::Ok().json(JsonResponse::simple_response())
} // end of start_party

/// Struct to represent the data sent to create a party
#[derive(Serialize, Deserialize)]
struct CreatePartyData {
    id: String,
    secret: String
} // end of CreatePartyData



#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i32
}
/// Controller to get the client id and the client secret and request the access token
#[post("/createParty")]
async fn request_token(req: HttpRequest, form: web::Form<CreatePartyData>) -> impl Responder {

    // check that the user is logged in
    if !check_login(req.headers()) {
        // not logged in, redirect
        let res = JsonResponse::new(false, true, String::from("http://localhost:3000/login"));
        return HttpResponse::Ok().json(res);
    }
    // the user is logged in, send the request to get the access token
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(10)))
        .finish();
    let to_concat = "grant_type=client_credentials&client_id=";
    let req_body = format!("{}{}{}{}", to_concat, form.id, "&client_secret=", form.secret );
    //println!("{}", req_body);
    let mut response = client.post("https://accounts.spotify.com/api/token").timeout(Duration::from_secs(10)).
    insert_header(("Content-Type", "application/x-www-form-urlencoded")).send_body(req_body).await.unwrap();
    //response.timeout(Duration::from_secs(10));
    println!("{:?}", response.version());
    println!("{:?}", response.status());
    let payload = response.json::<AccessToken>().await.unwrap();
    println!("Access token{}", payload.access_token);
    println!("{}", payload.token_type);
    println!("{}", payload.expires_in);
    //println!("{}", form.id);
    //println!("{}", form.secret);
    HttpResponse::Ok().json(JsonResponse::simple_response())
}