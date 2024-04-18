use std::collections::HashMap;

use actix_web::http::header::HeaderMap;
use actix_web::http::header::HeaderValue;

use super::cookie_parser::MyCookie;
use crate::utils::cookie_parser::*;

/// retrives the cookies hash map from the cookies http header
fn get_cookies_hash_map(header_map: &HeaderMap) -> Option<HashMap<String, MyCookie>> {
    let cookie_str = "Cookie";
    let log_token = header_map.get(cookie_str);
    // check if there are some cookies
    if log_token.is_none() {
        // no cookies
        return None;
    }

    let header_value: &HeaderValue = log_token.unwrap();
    //println!("{:?}", header_value);
    let header_str = header_value.to_str().unwrap();
    //println!("{:?}", header_str);
    // create the hashmapt with all the cookies
    let cookies = parse_cookies(header_str);
    Some(cookies)
}

/// Retrives the authorization token, that was sent as a cookie
pub fn get_authorization_token_cookie(header_map: &HeaderMap) -> Option<String> {
    // cookies hashmap
    let cookies = get_cookies_hash_map(header_map);
    if cookies.is_none() {
        return None;
    }
    let cookies = cookies.unwrap();
    if cookies.contains_key("access") {
        return Some(cookies.get("access").unwrap().value.clone());
    }
    // there is nothing
    None
}