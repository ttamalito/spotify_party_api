use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use crate::utils::refresh_token::refresh_token;
use super::put_request_empty::put_request_emtpy_body;

/// This function is responsible for refreshing the access token and then sending an empty PUT request.
///
/// # Arguments
///
/// * `req` - An instance of HttpRequest which contains the request data.
/// * `auth_header` - A reference to a string representing the authorization header value.
/// * `url` - A reference to a string representing the URL to send the PUT request to.
///
/// # Returns
///
/// A tuple containing a boolean and an HttpResponse.
/// The boolean indicates whether the token was refreshed successfully (true) or not (false).
/// The HttpResponse represents the response from the server after sending the PUT request.
/// If the token was refreshed successfully and the PUT request was successful, the HttpResponse will be NoContent (204).
/// If the token could not be refreshed or the PUT request failed, the HttpResponse will be BadRequest (400).
pub async fn refresh_and_send_empty_put_request(req: HttpRequest, auth_header: &str, url: &str) -> (bool, HttpResponse) {
    // try to refresh the token
    let refreshed = refresh_token(req).await;
    if refreshed {
        // token refreshed successfully
        let response_result_after_refreshed = put_request_emtpy_body(auth_header, url).await;
        if response_result_after_refreshed.0 {
            return (true, HttpResponse::NoContent().finish());
        } else {
            return (false, HttpResponse::BadRequest().finish());
        }
    }
    // token could not be refreshed

    (false, HttpResponse::BadRequest().finish())
}