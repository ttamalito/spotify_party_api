use actix_web::{get, http::{header::HeaderValue, StatusCode}, web::Redirect, HttpRequest, HttpResponse, Responder};
use crate::utils::cookie_parser::parse_cookies;


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
        println!("{:?}", header_value);
        let header_str = header_value.to_str().unwrap();
        println!("{:?}", header_str);
        // create an iterator
        parse_cookies(header_str);
    }
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