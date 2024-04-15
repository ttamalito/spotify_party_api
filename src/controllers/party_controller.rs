use actix_web::{get, http::{header::HeaderValue, StatusCode}, web::Redirect, HttpRequest, HttpResponse, Responder};
use jwt::Store;
use crate::utils::cookie_parser::parse_cookies;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

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
    println!("{:?}", header_map);
    let cookie_str = "Cookie";
    let log_token = header_map.get(cookie_str);
    if log_token.is_some() {
        let header_value: &HeaderValue = log_token.unwrap();
        //println!("{:?}", header_value);
        let header_str = header_value.to_str().unwrap();
        //println!("{:?}", header_str);
        // create an iterator
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
            // there is a token
            let token = cookies.get("jwt").unwrap().get_value_as_ref();

            let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").expect("Should generate the key");
            //let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";
            let claims: BTreeMap<String, String> = token.verify_with_key(&key).expect("Should verify key");
            //assert_eq!(claims["sub"], "someone");
            println!("{:?}", claims);
        }
    } // end of IF there are cookies

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