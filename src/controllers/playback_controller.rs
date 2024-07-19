use actix_web::{
    get, post, put,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
use awc::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::utils::check_login::check_login;
use crate::utils::response::JsonResponse;

use hmac::Mac;

use crate::application_data::*;

use crate::utils::build_headers::build_authorization_header::*;
use crate::utils::check_party_exists_and_user_is_owner::*;
use crate::utils::refresh_token::refresh_token;
use crate::utils::requests_to_api::inital_check_for_users::intial_checkup;
use crate::utils::requests_to_api::post_request_empty::post_request_emtpy_body;
use crate::utils::requests_to_api::put_request_empty::put_request_emtpy_body;
use crate::utils::requests_to_api::refresh_post_empty_body::refresh_and_send_post_empty_body;
use crate::utils::requests_to_api::refresh_put_request_empty::refresh_and_send_empty_put_request;
use crate::utils::requests_to_api::get_request::get_request_get_play_back_state;
use crate::utils::requests_to_api::refresh_and_send_get_playback_state::refresh_and_send_get_playback_state;
// struct for pausePlayback
#[derive(Deserialize, Serialize)]
struct PausePlaybackForm {
    party_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ErrorSpotify {
    //#[serde(skip_deserializing)]
    status: i32,
    message: String,
}
/// Struct for deserializing Error From Spotify API
#[derive(Deserialize, Serialize, Debug)]
struct MainError {
    error: ErrorSpotify,
}

#[derive(Deserialize, Serialize, Debug)]
struct MainError2 {
    ContentType: MainError,
}

/// Controller to pause the playback
#[post("/pausePlayback")]
async fn pause_playback(req: HttpRequest, _form: web::Form<PausePlaybackForm>) -> impl Responder {
    let req_clone = req.clone();
    // initial checkup
    let (response, possible_auth_header) = intial_checkup(req).await;
    if possible_auth_header.is_none() {
        // not authorized
        return response;
    }
    // convert the auth header as a str
    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    let url = "https://api.spotify.com/v1/me/player/pause";
    let response_result = put_request_emtpy_body(auth_header, url).await;
    if response_result.0 {
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        println!("{}", "You need to refresh your token");
        let (_result, response_to_send) =
            refresh_and_send_empty_put_request(req_clone, auth_header, url).await;
        return response_to_send;
    }

    HttpResponse::BadRequest().finish()
} // end of pause_playback

/// Controller to resume Playback
#[get("/resumePlayback")]
async fn resume_playback(req: HttpRequest) -> impl Responder {
    let req_clone = req.clone();
    // initial checkup
    let (response, possible_auth_header) = intial_checkup(req).await;
    if possible_auth_header.is_none() {
        // not authorized
        return response;
    }
    // convert the auth header as a str
    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    let url = "https://api.spotify.com/v1/me/player/play";
    let response_result = put_request_emtpy_body(auth_header, url).await;

    if response_result.0 {
        // all good
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        println!("{}", "You need to refresh your token");
        let (_result, response_to_send) =
            refresh_and_send_empty_put_request(req_clone, auth_header, url).await;
        return response_to_send;
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
        let (_result, response_to_send) =
            refresh_and_send_post_empty_body(req_clone, auth_header, url).await;
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
        let (_result, response_to_send) =
            refresh_and_send_post_empty_body(req_clone, auth_header, url).await;
        // to be honest, there is not need to read the result, just send the response
        return response_to_send;
    }

    HttpResponse::BadRequest().finish()
}
/// Controller to modify the volume of the current playback
///
/// # Arguments
///
/// * `req` - The HttpRequest object containing the request details
///
/// # Returns
///
/// * An HttpResponse object with the appropriate status code and body
#[get("/modifyVolume")]
async fn modify_volume(req: HttpRequest) -> impl Responder {
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
    let url = "https://api.spotify.com/v1/me/player/volume?volume_percent=2";
    // send the request to the api
    let response_result = put_request_emtpy_body(auth_header, url).await;
    if response_result.0 {
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        println!("{}", "You need to refresh your token");
        let (_result, response_to_send) =
            refresh_and_send_empty_put_request(req_clone, auth_header, url).await;
        return response_to_send;
    }

    HttpResponse::BadRequest().finish()
}

/// Controller to turn on the shuffle mode
#[put("/shuffleOn")]
async fn turn_on_shuffle(req: HttpRequest) -> impl Responder {
    let req_clone = req.clone();
    let (response, possible_auth_header) = intial_checkup(req).await;
    if possible_auth_header.is_none() {
        return response;
    }

    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    let url = "https://api.spotify.com/v1/me/player/shuffle?state=true";

    // send the request
    let response_result = put_request_emtpy_body(auth_header, url).await;
    if response_result.0 {
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        println!("{}", "You need to refresh your token");
        let (_result, response_to_send) =
            refresh_and_send_empty_put_request(req_clone, auth_header, url).await;
        return response_to_send;
    }

    HttpResponse::BadRequest().finish()
} // end of shuffleOn

/// Controller to turn the shuffle mode off
#[put("/shuffleOff")]
async fn turn_off_shuffle(req: HttpRequest) -> impl Responder {
    let req_clone = req.clone();
    let (response, possible_auth_header) = intial_checkup(req).await;
    if possible_auth_header.is_none() {
        return response;
    }

    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    let url = "https://api.spotify.com/v1/me/player/shuffle?state=false";

    // send the request
    let response_result = put_request_emtpy_body(auth_header, url).await;
    if response_result.0 {
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        println!("{}", "You need to refresh your token");
        let (_result, response_to_send) =
            refresh_and_send_empty_put_request(req_clone, auth_header, url).await;
        return response_to_send;
    }

    HttpResponse::BadRequest().finish()
} // end of shuffleOff


#[get("/getPlaybackState")]
async fn get_playback_state(req: HttpRequest) -> impl Responder {
    let req_clone = req.clone();
    let (response, possible_auth_header) = intial_checkup(req).await;
    if possible_auth_header.is_none() {
        return response;
    }

    let auth_header = possible_auth_header.unwrap();
    let auth_header = auth_header.as_str();
    let url = "https://api.spotify.com/v1/me/player";
    let response_result = get_request_get_play_back_state(auth_header, url).await;
    if response_result.1 == StatusCode::OK {
        // return the data
        if response_result.2.is_some() {
            let response = response_result.2.unwrap();
            return HttpResponse::Ok().json(response);
        } else {
            let response = response_result.3.unwrap();
            return HttpResponse::Ok().json(response);
        }
    } else if response_result.1 == StatusCode::NO_CONTENT {
        return HttpResponse::NoContent().finish();
    } else if response_result.1 == StatusCode::UNAUTHORIZED {
        println!("{}", "You need to refresh your token");
        let (result, status, trackData, episodeData) =
            refresh_and_send_get_playback_state(req_clone, auth_header, url).await;
        if result {
            if trackData.is_some() {
                return HttpResponse::Ok().json(trackData.unwrap());
            } else if episodeData.is_some() {
                return HttpResponse::Ok().json(episodeData.unwrap());
            } else {
                // NO data
                return HttpResponse::NoContent().finish();
            }
        }
        // else send a bad request
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::BadRequest().finish()
        
    
} // end of get_playback_state




