use actix_web::{HttpRequest, HttpResponse};
use crate::utils::refresh_token::refresh_token;
use super::post_request_empty::post_request_emtpy_body;
use super::get_request::get_request_get_play_back_state;
use awc::http::StatusCode;
use crate::utils::structs_to_serialize_deserialize::{MainResponsePlaybackStateEpisodeObjectItem, MainResponsePlaybackStateTrackObjectItem};
/// refreshes a token and sends a post request with empty body to the given url
pub async fn refresh_and_send_get_playback_state(req: HttpRequest, auth_header: &str, url: &str) -> (bool, StatusCode, Option<MainResponsePlaybackStateTrackObjectItem>, Option<MainResponsePlaybackStateEpisodeObjectItem>) {
    
    // try to refresh the token
    let refreshed = refresh_token(req).await;
    if refreshed {
        // send the request again
        let response_result_after_refreshed = get_request_get_play_back_state(auth_header, url).await;
        if response_result_after_refreshed.0 {
            return (true, response_result_after_refreshed.1, response_result_after_refreshed.2, response_result_after_refreshed.3);
        } else {
            return (false, StatusCode::BAD_REQUEST, None, None);
        }
    }
    println!("Token could not be refreshed");

    (false, StatusCode::BAD_REQUEST, None, None)
}