use actix_web::{HttpRequest, HttpResponse};
use crate::utils::refresh_token::refresh_token;
use super::post_request_empty::post_request_emtpy_body;


/// refreshes a token and sends a post request with empty body to the given url
pub async fn refresh_and_send_post_empty_body(req: HttpRequest, auth_header: &str, url: &str) -> (bool, HttpResponse) {
    
    // try to refresh the token
    let refreshed = refresh_token(req).await;
    if refreshed {
        println!("Token refreshed successfully");
        // send the request again
        let response_result_after_refreshed = post_request_emtpy_body(auth_header, url).await;
        if response_result_after_refreshed.0 {
            return (true, HttpResponse::NoContent().finish());
        } else {
            return (false, HttpResponse::BadRequest().finish());
        }
    }
    println!("Token could not be refreshed");

    (false, HttpResponse::BadRequest().finish())
}