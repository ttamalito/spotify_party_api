use actix_web::{get, http::{header::HeaderValue, StatusCode}, post, web::{self, Json, Redirect, Data}, HttpRequest, HttpResponse, Responder};
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
use crate::models::party_model::*;
use crate::models::user_model::*;
use crate::application_data::*;
use mongodb::bson::oid::ObjectId;

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
    if !check_login(header_map).0 {
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


/// Struct to deserialize the data of the spotify api, for the request token
#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i32
}


#[derive(Deserialize, Serialize)]
struct JsonResponseForWithAccessToken {
    result: bool,
    redirected: bool,
    url: String,
    access_token: String,
    token_type: String,
    expires_in: i32
}

impl JsonResponseForWithAccessToken {
    pub fn new(json: JsonResponse, token: AccessToken) -> Self {
        JsonResponseForWithAccessToken {
            result: json.get_result(),
            redirected: json.get_redirected(),
            url: json.get_url(),
            access_token: token.access_token,
            token_type: token.token_type,
            expires_in: token.expires_in
        }
    }
}

/// Controller to get the client id and the client secret and request the access token
#[post("/createParty")]
async fn request_token(req: HttpRequest, form: web::Form<CreatePartyData>) -> impl Responder {

    // check that the user is logged in
    let (logged, user_id) = check_login(req.headers());
    if !logged {
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
    // create a party and save it to the data base
    let party = Party::new(user_id.clone(), payload.access_token.clone(), payload.token_type.clone(), payload.expires_in.clone());
    let collection = PartyCollection::new(req.app_data::<Data<ApplicationData>>());
    // save it to the database
    let party_id = collection.save_party(party).await;
    // save it to the database
    let user_collection = User::new(req.app_data::<Data<ApplicationData>>());
    // convert the string to objectid
    let user_id = ObjectId::parse_str(user_id).unwrap();
    let all_good = user_collection.add_owned_party(user_id, party_id).await;
    if all_good {
        return HttpResponse::Ok().json(JsonResponseForWithAccessToken::new(JsonResponse::simple_response(), payload));
    }

    HttpResponse::InternalServerError().json(JsonResponse::new(false, false, String::from("")))
    
}