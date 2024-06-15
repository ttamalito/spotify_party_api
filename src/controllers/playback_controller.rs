use awc::http::StatusCode;
use serde::{Deserialize, Serialize};
use actix_web::{get, post, web::{self, Data}, HttpRequest, HttpResponse, Responder};

use crate::utils::{check_login::check_login};
use crate::utils::{response::JsonResponse};


use hmac::{Mac};







use crate::application_data::*;

use crate::utils::check_party_exists_and_user_is_owner::*;
use crate::utils::build_headers::build_authorization_header::*;
use crate::utils::requests_to_api::put_request_empty::put_request_emtpy_body;
use crate::utils::requests_to_api::post_request_empty::post_request_emtpy_body;
use crate::utils::requests_to_api::inital_check_for_users::intial_checkup;
use crate::utils::refresh_token::refresh_token;
use crate::utils::requests_to_api::refresh_post_empty_body::refresh_and_send_post_empty_body;

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
async fn pause_playback(req: HttpRequest, _form: web::Form<PausePlaybackForm> ) -> impl Responder {
    // check that the user is logged in
    let (logged, user_id) = check_login(req.headers());

    if !logged {
        // not logged in
        return HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login());
    }

    // check that the user owns the party and that the party exists
    let (_is_owner, _response, possible_access_token) = check_party_exists_and_user_is_owner_method(&user_id, req.app_data::<Data<ApplicationData>>()).await;
    

    // now send the corresponding https request to pause the playback
    // authorization header

    // there is a token, create the &str
    let (auth_token_exists, auth_header) = get_authorization_header(req.headers(), possible_access_token);
    // check that the token existed in the cookies
    if !auth_token_exists {
        // there is no token
        return HttpResponse::Unauthorized().
        json(JsonResponse::new(false, true, String::from("http://localhost:3000/startParty")));  // TODO --- change the url to redirect, to refresh the token
    }

    let auth_header = auth_header.as_str();
    let response_result = put_request_emtpy_body(auth_header, "https://api.spotify.com/v1/me/player/pause").await;


    //let payload = response.json::<serde_json::Value>().await.expect("What ever");
    if response_result {
        return HttpResponse::NoContent().finish();
    }

    HttpResponse::BadRequest().finish()
} // end of pause_playback

/// Controller to resume Playback
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
        let (is_owner, response, possible_access_token) = check_party_exists_and_user_is_owner_method(&user_id, req.app_data::<Data<ApplicationData>>()).await;

        if !is_owner {
            return response; // if not the owenr send the corresponding response
        }

        // get the authorization header
        let (exists_token, auth_header) = get_authorization_header(req.headers(), possible_access_token);
        if !exists_token {
            // there is no token
            return HttpResponse::Unauthorized().
            json(JsonResponse::new(false, true, String::from("http://localhost:3000/startParty")));  // TODO --- change the url to redirect, to refresh the token
        }


        // convert the auth header as a str
        let auth_header = auth_header.as_str();
        // send the request to the api
        let response_result = put_request_emtpy_body(auth_header, "https://api.spotify.com/v1/me/player/play").await;
        
        if response_result {
            // all good 
            return HttpResponse::NoContent().finish();
        }
        // else send a bad request response
        HttpResponse::BadRequest().finish()

}


#[get("/playNext")]
async fn play_next(req: HttpRequest) -> impl Responder {
    let req_clone = req.clone();
    
    let (response, possible_auth_header) = intial_checkup(req).await;
    // check if there is an authorization header
    if possible_auth_header.is_none() {
        // not authorized
        return response;
    }
    // convert the auth header as a str
    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    let url = "https://api.spotify.com/v1/me/player/next";
    // send the request to the api
    let response_result = post_request_emtpy_body(auth_header, url).await;

    if response_result.0 {
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        // try to refresh the token and send the request again
        let (_result, response_to_send) = refresh_and_send_post_empty_body(req_clone, auth_header, url).await;
        // to be honest, there is not need to read the result, just send the response
        return response_to_send;
    }

    HttpResponse::BadRequest().finish()
}

#[get("/playPrevious")]
async fn play_previous(req: HttpRequest) -> impl Responder {
    let req_clone = req.clone();
    let (response, possible_auth_header) = intial_checkup(req).await;

    // check if there is an authorization header
    if possible_auth_header.is_none() {
        // not authorized
        return response;
    }
    // convert the auth header as a str
    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    // send the request to the api
    let url = "https://api.spotify.com/v1/me/player/previous";
    let response_result = post_request_emtpy_body(auth_header, url).await;
    if response_result.0 {
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        // try to refresh the token and send the request again
        let (_result, response_to_send) = refresh_and_send_post_empty_body(req_clone, auth_header, url).await;
        // to be honest, there is not need to read the result, just send the response
        return response_to_send;

    }

    HttpResponse::BadRequest().finish()
}

#[get("/modifyVolume")]
async fn modify_volume(req: HttpRequest) -> impl Responder {
    let (response, possible_auth_header) = intial_checkup(req).await;

    // check if there is an authorization header
    if possible_auth_header.is_none() {
        // not authorized
        return response;
    }
    // convert the auth header as a str
    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    // send the request to the api
    let response_result = put_request_emtpy_body(auth_header, "https://api.spotify.com/v1/me/player/volume?volume_percent=2").await;
    if response_result {
        return HttpResponse::NoContent().finish();
    }

    HttpResponse::BadRequest().finish()
}