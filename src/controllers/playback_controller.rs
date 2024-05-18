use serde::{Deserialize, Serialize};
use actix_web::{get, http::{header::{ContentType, HeaderValue}, StatusCode}, post, put, web::{self, Data, Json, Redirect}, HttpRequest, HttpResponse, Responder};
use jwt::Store;
use crate::utils::{check_login::check_login, check_party_exists_and_user_is_owner, response};
use crate::utils::{response::JsonResponse};
use awc::{Connector, Client};
use openssl::ssl::{SslConnector, SslMethod};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use mongodb::bson::oid::ObjectId;
use std::{cmp::Ordering, collections::BTreeMap, str::FromStr, time::Duration};
use crate::utils::get_cookie::*;
use crate::models::party_model::*;
use crate::models::user_model::*;
use crate::application_data::*;
use crate::utils::convert_to_object_id::convert_to_object_id;
use crate::utils::check_party_exists_and_user_is_owner::*;
use crate::utils::build_headers::build_authorization_header::*;


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
/// Struct for deserializing Error From Spotify API
#[derive(Deserialize, Serialize, Debug)]
struct MainError {
    error: ErrorSpotify
}

#[derive(Deserialize, Serialize, Debug)]
struct MainError2 {
    ContentType: MainError
}

/// Controller to pause the playback
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
    println!("{:?}", auth_token);
    // there is a token, create the &str
    let auth_header = format!("{}{}", "Bearer ", auth_token.unwrap());
    let auth_header = auth_header.as_str();
    println!("{:?}", auth_header);
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(54)))
        .finish();

    //println!("{}", req_body);
    let mut response = client.put("https://api.spotify.com/v1/me/player/pause").timeout(Duration::from_secs(45)).
    insert_header(("Authorization", auth_header))
    .send().await.unwrap();
    println!("{:?}", response.headers());
    // check the response code
    println!("{:?}", response.version());
    println!("{:?}", response.status());
    if response.status() == StatusCode::NO_CONTENT {
        // all good

        
        return HttpResponse::NoContent().finish();
    }
    //let payload = response.json::<serde_json::Value>().await.expect("What ever");
    let payload = response.json::<MainError>().await.expect("Should deserialize");
    println!("{:?}", payload);
    HttpResponse::BadRequest().finish()
} // end of pause_playback

#[get("/resumePlayback")]
async fn resume_playback(req: HttpRequest) -> impl Responder {

    // first check that the user is logged in
        // check that the user is logged in
        let (logged, user_id) = check_login(req.headers());

        if !logged {
            // not logged in
            return HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login());
        }

        // check that the user is the owner of the party
        let (is_owner, response) = check_party_exists_and_user_is_owner_method(&user_id, req.app_data::<Data<ApplicationData>>()).await;

        if !is_owner {
            return response; // if not the owenr send the corresponding response
        }

        // get the authorization header
        let (exists_token, auth_header) = get_authorization_header(req.headers());
        if !exists_token {
            // there is no token
            return HttpResponse::Unauthorized().
            json(JsonResponse::new(false, true, String::from("http://localhost:3000/startParty")));  // TODO --- change the url to redirect, to refresh the token
        }


        // convert the auth header as a str
        let auth_header = auth_header.as_str();
        // send the request to the api
        let builder = SslConnector::builder(SslMethod::tls()).unwrap();

        let client = Client::builder()
            .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(54)))
            .finish();
    
        //println!("{}", req_body);
        let mut response = client.put("https://api.spotify.com/v1/me/player/play").timeout(Duration::from_secs(45)).
        insert_header(("Authorization", auth_header))
        .send().await.unwrap();
        println!("{:?}", response.headers());
        // check the response code
        println!("{:?}", response.version());
        println!("{:?}", response.status());
        if response.status() == StatusCode::NO_CONTENT {
            // all good
    
            
            return HttpResponse::NoContent().finish();
        }
        //let payload = response.json::<serde_json::Value>().await.expect("What ever");
        let payload = response.json::<MainError>().await.expect("Should deserialize");
        println!("{:?}", payload);
        HttpResponse::BadRequest().finish()

}

#[get("/playNext")]
async fn play_next(req: HttpRequest) -> impl Responder {

    let (logged, user_id) = check_login(req.headers());

    if !logged {
        // not logged in
        return HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login());
    }

    // check that the user is the owner of the party
    let (is_owner, response) = check_party_exists_and_user_is_owner_method(&user_id, req.app_data::<Data<ApplicationData>>()).await;

    if !is_owner {
        return response; // if not the owenr send the corresponding response
    }

    // get the authorization header
    let (exists_token, auth_header) = get_authorization_header(req.headers());
    if !exists_token {
        // there is no token
        return HttpResponse::Unauthorized().
        json(JsonResponse::new(false, true, String::from("http://localhost:3000/startParty")));  // TODO --- change the url to redirect, to refresh the token
    }


    // convert the auth header as a str
    let auth_header = auth_header.as_str();
    // send the request to the api
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(54)))
        .finish();

    //println!("{}", req_body);
    let mut response = client.post("https://api.spotify.com/v1/me/player/next").timeout(Duration::from_secs(45)).
    insert_header(("Authorization", auth_header))
    .send().await.unwrap();
    println!("{:?}", response.headers());
    // check the response code
    println!("{:?}", response.version());
    println!("{:?}", response.status());
    if response.status() == StatusCode::NO_CONTENT {
        // all good

        
        return HttpResponse::NoContent().finish();
    }
    //let payload = response.json::<serde_json::Value>().await.expect("What ever");
    let payload = response.json::<MainError>().await.expect("Should deserialize");
    println!("{:?}", payload);
    HttpResponse::BadRequest().finish()
}