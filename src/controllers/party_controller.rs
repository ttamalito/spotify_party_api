use actix_web::{get, http::{header::{ContentType, HeaderValue}, StatusCode}, post, put, web::{self, Data, Json, Redirect}, HttpRequest, HttpResponse, Responder};
use jwt::Store;
use crate::utils::{cookie_parser::parse_cookies, response::JsonResponseWithLengthOfQueue};
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
use crate::utils::structs_to_serialize_deserialize::*;
use serde_json;
use crate::utils::convert_to_object_id::convert_to_object_id;



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
        .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(20)))
        .finish();

    //println!("{}", req_body);
    let mut response = client.post("https://accounts.spotify.com/api/token").timeout(Duration::from_secs(20)).
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
        .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(20)))
        .finish();

    //println!("{}", req_body);
    let mut response = client.put("https://api.spotify.com/v1/me/player/pause").timeout(Duration::from_secs(20)).
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

/// Controller to request to join a party
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

    // check if the user has requested to join the party already
    for user in party.get_requested_to_join_as_ref() {
        let ordering = user_id.cmp(user);
        //println!("{:?}", ordering);
        if ordering.is_eq() {
            // the user has already requested to join the party
            return HttpResponse::ImATeapot().json(JsonResponse::new(false, false, String::from("User requested to join the party already")));
        }
    }

    // else add the user to the people that have requested to join the party
    let res = party_collection.insert_requested_to_join(party_id, user_id).await;
    if !res {
        return HttpResponse::Forbidden().json(JsonResponse::new(false, false, String::from("Something went wrong while inserting the user to the queueof the  party")));
    }

    /*
        let insertion_result = party_collection.insert_member(party_id, user_id).await;
    if !insertion_result {
        return HttpResponse::Forbidden().json(JsonResponse::new(false, false, String::from("Something went wrong while inserting the user to the party")));
    }
     */
    // else all good
    HttpResponse::Ok().json(JsonResponse::simple_response())
}

/// get the amount of people that want to join the party
#[get("/getQueueToJoinLength/{id}")]
async fn get_length_to_join_queue(req: HttpRequest) -> impl Responder {
    // check that the user is logged in
    let (logged, user_id) = check_login(req.headers());
    if !logged {
        // not logged in
        return HttpResponse::Forbidden().json(JsonResponse::redirect_to_login());
    }

    // get the party_id
    let match_info_data = req.match_info();
    let party_id = match_info_data.get("id").unwrap();

    // get the party
    let party_collection = PartyCollection::new(req.app_data::<Data<ApplicationData>>());

    // check if it can be converted to OBjectId
    let possible_party_id = ObjectId::parse_str(party_id);
    if possible_party_id.is_err() {
        // not possible
        return HttpResponse::BadRequest().json(JsonResponse::new(false, false, String::from("Not possible to convert the id to an ObjectId")));
    }

    // else all good
    let party_id = possible_party_id.unwrap();
    // get the party from the database
    let party = party_collection.query_by_id(party_id).await.unwrap();
    if party.is_none() {
        // there is no party
        return HttpResponse::BadRequest().json(JsonResponse::new(false, false, String::from("No party with that id")));
    }
    let party = party.unwrap();

    // check if the user is the owner of the party
    let owner_of_party = party.owner;    
    let user_id = ObjectId::parse_str(user_id).unwrap();
    let ordering = user_id.cmp(&owner_of_party);
    if ordering.is_ne() {
        // not equal
        return HttpResponse::BadRequest().json(JsonResponse::new(false, false, String::from("You are not the owner of the party")));
    }

    // else all good
    // get the length of the queue
    let length = party.get_requested_to_join_as_ref().len();
    // send the json response with the corresponding length
    let response = JsonResponseWithLengthOfQueue::new(true, false, String::from("All good"), length);

    HttpResponse::Ok().json(response)
}

/// controller to accept a user into the party
#[put("/acceptIntoParty/{user}/{party}")]
async fn acceptIntoParty(req: HttpRequest) -> impl Responder {
    // check if the user is logged in
    let (logged, user_id) = check_login(req.headers());

    if !logged {
        // not logged in
        return HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login());
    }

    // ok, now convert the id into object id
    let match_info_data = req.match_info();
    let party_id = match_info_data.get("party").unwrap();
    let user_to_accept = match_info_data.get("user").unwrap();

    // convert them into object id

    HttpResponse::Ok().finish()
}

/// Controller to retrieve all the user that want to join a party
#[get("/getQueueToJoinParty/{party}")]
async fn getQueueToJoinParty(req: HttpRequest) -> impl Responder {

    // check that the user is logged in
    let (logged, user_id) = check_login(req.headers());
    if !logged {
        // user is not logged in
        return HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login())
    }

    // get the party_id
    let match_info_data = req.match_info();
    let party_id = match_info_data.get("party").unwrap();

    // check that the party exists
    // see if it can be converted to objectId
    let possible_party_id = ObjectId::parse_str(party_id);
    if possible_party_id.is_err() {
        // not possible
        return HttpResponse::BadRequest().json(JsonResponse::new(false, false, String::from("Not possible to convert the id to an ObjectId")));
    }

    // else all good
    let party_id = possible_party_id.unwrap();

    // get the collection
    let party_collection = PartyCollection::new(req.app_data::<Data<ApplicationData>>());

    // query the database
    let possible_party = party_collection.query_by_id(party_id).await.expect("Should query the database");

    if possible_party.is_none() {
        // there is no party with that id
        return HttpResponse::BadRequest().json(JsonResponse::new(false, false, String::from("No party with that id")))
    }

    // otherwise there is a party
    let party = possible_party.unwrap();

    // check that the user is the owner of the party
    let owner = party.owner;
    let user_id = ObjectId::parse_str(user_id).unwrap();
    let comparison = owner.cmp(&user_id);
    if comparison.is_ne() {
        // not the owner
        return HttpResponse::Unauthorized().json(JsonResponse::new(false, false, String::from("User is not the owner of the paryt")));
    }

    // ok so all good at this point, return the list of users wanting to join
    let users_wanting_to_join = party.get_requested_to_join_as_ref();

    let response = ResponseForQueueToJoinParty {
        result: true,
        message: String::from("All gucci"),
        users: users_wanting_to_join.to_owned()
    };



    HttpResponse::Ok().json(response)
}