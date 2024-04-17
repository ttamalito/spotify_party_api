use actix_web::http::header::HeaderMap;
use actix_web::http::header::HeaderValue;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::utils::cookie_parser::parse_cookies;

/// Checks if a user is logged in
/// Returns true if the user is logged in, false otherwise
pub fn check_login(header_map: &HeaderMap) -> (bool, String) {
    let cookie_str = "Cookie";
    let log_token = header_map.get(cookie_str);
    // check if there are some cookies
    if log_token.is_none() {
        // no cookies
        return (false, String::from("no jwt cookie"));
    }
    // check that the jwt cookie is present
    let header_value: &HeaderValue = log_token.unwrap();
    //println!("{:?}", header_value);
    let header_str = header_value.to_str().unwrap();
    //println!("{:?}", header_str);
    // create the hashmapt with all the cookies
    let cookies = parse_cookies(header_str);
    /*
            for cookie in cookies {
        println!("{}", String::from("Key"));
        println!("{}", cookie.get_key_as_ref());
        println!("{}", String::from("Value"));
        let value = cookie.get_value_as_ref();
        println!("{}", value);
    } */
    // check that the jwt cookie is present
    if cookies.contains_key("jwt") {
        // there is a jwt token
        let token = cookies.get("jwt").unwrap().get_value_as_ref();

        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").expect("Should generate the key");
        //let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";
        let claims: BTreeMap<String, String> = token.verify_with_key(&key).expect("Should verify key");
        //assert_eq!(claims["sub"], "someone");
        println!("{:?}", claims);

        return (true, claims["id"].to_string());
    } else {
        // there is no jwt
        println!("{}", String::from("About to redirect because no jwt cookie"));
        return (false, String::from("no jwt cookie"));
    }
}