use awc::http::header::HeaderMap;
use super::super::get_cookie::*;

/// Retrieves the auth token from the cookie and creates the authorization header
pub fn get_authorization_header(headers: &HeaderMap) -> (bool, String) {

    let auth_token = get_authorization_token_cookie(headers);

    if auth_token.is_none() {
        // there is no token
        return (false, String::from("There is no header with the token"));
    }
    println!("{:?}", auth_token);
    // there is a token, create the &str
    let auth_header = format!("{}{}", "Bearer ", auth_token.unwrap());
    //let auth_header = auth_header.as_str();
    println!("{:?}", &auth_header);

    (true, auth_header)
}