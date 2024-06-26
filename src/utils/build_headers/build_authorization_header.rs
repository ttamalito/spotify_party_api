use awc::http::header::HeaderMap;
use super::super::get_cookie::*;

/// Retrieves the auth token from the cookie and creates the authorization header
pub fn get_authorization_header(headers: &HeaderMap, possible_access_token: Option<String>) -> (bool, String) {

    let mut auth_token = get_authorization_token_cookie(headers);

    if auth_token.is_none() && possible_access_token.is_none() {
        // there is no token
        return (false, String::from("There is no header with the token"));
    }

    if auth_token.is_none() && possible_access_token.is_some() {
        auth_token = possible_access_token;
    }
    // there is a token, create the &str
    let auth_header = format!("{}{}", "Bearer ", auth_token.unwrap());


    (true, auth_header)
}