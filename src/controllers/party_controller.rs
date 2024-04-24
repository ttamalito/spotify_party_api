use actix_web::{get, http::{header::HeaderValue, StatusCode}, post, web::{self, Json, Redirect, Data}, HttpRequest, HttpResponse, Responder};
use jwt::Store;
use crate::utils::cookie_parser::parse_cookies;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::{cmp::Ordering, collections::BTreeMap, str::FromStr, time::Duration};
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
use base64::prelude::*;
use crate::utils::get_cookie::*;
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
    secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String
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
    expires_in: i32,
    party_id: String
}

impl JsonResponseForWithAccessToken {
    pub fn new(json: JsonResponse, token: AccessToken, party_id: String) -> Self {
        JsonResponseForWithAccessToken {
            result: json.get_result(),
            redirected: json.get_redirected(),
            url: json.get_url(),
            access_token: token.access_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
            party_id: party_id
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
    // check that the user doesnot own another party
    let optional_party = PartyCollection::new(req.app_data::<Data<ApplicationData>>()).query_by_owner(ObjectId::from_str(user_id.clone().as_str()).unwrap()).await;
    if optional_party.is_some() {
        // user already has a party
        return HttpResponse::Forbidden().json(JsonResponse::new(false, false, String::from("")));
    }
    // the user is logged in and doesn't have a party, send the request to get the access token
    let to_concat = format!("{}{}{}{}", "code=", form.code, "&grant_type=", form.grant_type);
    let req_body = format!("{}{}{}", to_concat, "&redirect_uri=", form.redirect_uri);
    println!("{}", &req_body);
    let authorization_header = format!("{}{}{}", form.id, ":", form.secret);
    println!("{}", &authorization_header);
    let base64_authorization_header = format!("{}{}", "Basic ", BASE64_STANDARD.encode(authorization_header));
    // send the request
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(5)))
        .finish();

    //println!("{}", req_body);
    let mut response = client.post("https://accounts.spotify.com/api/token").timeout(Duration::from_secs(5)).
    insert_header(("Content-Type", "application/x-www-form-urlencoded"))
    .insert_header(("Authorization", base64_authorization_header))
    .send_body(req_body).await.unwrap();
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
        // convert the party id to a string
        let party_id = party_id.to_hex();
        return HttpResponse::Ok().json(JsonResponseForWithAccessToken::new(JsonResponse::simple_response(), payload, party_id));
    }

    HttpResponse::InternalServerError().json(JsonResponse::new(false, false, String::from("")))
    
}


// struct for pausePlayback
#[derive(Deserialize, Serialize)]
struct PausePlaybackForm {
    party_id: String
}

#[derive(Deserialize, Serialize, Debug)]
struct ErrorSpotify {
    //#[serde(skip_deserializing)]
    status: i32,
    message: String
}
#[derive(Deserialize, Serialize, Debug)]
struct MainError {
    error: ErrorSpotify
}

#[post("/pausePlayback")]
async fn pause_playback(req: HttpRequest, form: web::Form<PausePlaybackForm> ) -> impl Responder {
    // check that the user is logged in
    let (logged, user_id) = check_login(req.headers());

    if !logged {
        // not logged in
        return HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login());
    }

    // check that the user owns the party and that the party exists
    let user_id = ObjectId::parse_str(user_id).expect("Should parse object id");

    let party_collection = PartyCollection::new(req.app_data::<Data<ApplicationData>>());
    let party = party_collection.query_by_owner(user_id).await;
    if party.is_none() {
        return HttpResponse::Conflict().json(JsonResponse::new(false, false, String::from("")));
    }
    // else there is a party, check that the user is the owner of that party
    let user_collection = User::new(req.app_data::<Data<ApplicationData>>());
    let user = user_collection.query_user_by_id(user_id).await;
    if user.is_none() {
        return HttpResponse::Conflict().json(JsonResponse::new(false, false, String::from("")));
    } else {
        let user = user.unwrap();
        let owned = user.owned_party;
        if owned.is_none() {
            return HttpResponse::Conflict().json(JsonResponse::new(false, false, String::from("")));
        } else if owned.unwrap().to_string() != form.party_id {
            return HttpResponse::Conflict().json(JsonResponse::new(false, false, String::from("Not the owner of the pary")));
        }
    }

    // now send the corresponding https request to pause the playback
    // authorization header
    let auth_token = get_authorization_token_cookie(req.headers());
    if auth_token.is_none() {
        return HttpResponse::Unauthorized().json(JsonResponse::new(false, true, String::from("http://localhost:3000/startParty"))); // TODO --- change the url to redirect, to refresh the token
    }
    // there is a token, create the &str
    let auth_header = format!("{}{}", "Bearer ", auth_token.unwrap());
    let auth_header = auth_header.as_str();
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(10)))
        .finish();

    //println!("{}", req_body);
    let mut response = client.put("https://api.spotify.com/v1/me/player/pause").timeout(Duration::from_secs(5)).
    insert_header(("Authorization", auth_header))
    .send().await.unwrap();
    // check the response code
    println!("{:?}", response.version());
    println!("{:?}", response.status());
    if response.status() == StatusCode::NO_CONTENT {
        // all good
        return HttpResponse::NoContent().finish();
    }
    let payload = response.json::<MainError>().await.expect("Should deserialize");
    println!("{:?}", payload);
    HttpResponse::BadRequest().finish()
} // end of pause_playback


#[get("/joinParty/{id}")]
async fn join_party(req: HttpRequest) -> impl Responder {

    // check if the user is logged in
    let (logged, user_id) = check_login(req.headers());
    if !logged {
        return HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login());
    }

    // get the party data
    let match_info_data = req.match_info();
    let party_id = match_info_data.get("id").unwrap();

    //println!("{:?}", party_id);

    // now check that the party exists
    let party_collection = PartyCollection::new(req.app_data::<Data<ApplicationData>>());
    let party_id = ObjectId::parse_str(party_id);
    if party_id.is_err() {
        return HttpResponse::Forbidden().json(JsonResponse::new(false, false, String::from("Not a valid Object Id")));
    }
    let party_id = party_id.unwrap();
    let party = party_collection.query_by_id(party_id).await.expect("Could not query the database");

    if party.is_none() {
        // there is no party
        return HttpResponse::Forbidden().json(JsonResponse::new(false, false, String::from("There is no party with that id")));
    }

    // there is a party, check if the user is already a memeber of the party
    let party = party.unwrap();
    let members = party.get_members_as_ref();
    // check if the user is part of the memebers of the party
    let user_id = ObjectId::parse_str(user_id).unwrap();
    for user in members {
        let ordering = user_id.cmp(user);
        println!("{:?}", ordering);
        if ordering.is_eq() {
            // the user is already a memeber of the party
            return HttpResponse::ImATeapot().json(JsonResponse::new(false, false, String::from("User is a memebr of the party already")));
        }
    }

    // else add the user to the party
    let insertion_result = party_collection.insert_member(party_id, user_id).await;
    if !insertion_result {
        return HttpResponse::Forbidden().json(JsonResponse::new(false, false, String::from("Something went wrong while inserting the user to the party")));
    }
    // else all good
    HttpResponse::Ok().json(JsonResponse::simple_response())
}