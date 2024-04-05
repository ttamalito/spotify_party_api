use actix_web::{get, http::StatusCode, web::Redirect, HttpRequest, HttpResponse, Responder};



#[get("/party")]
async fn start_party(req: HttpRequest) -> impl Responder {
    let header_map = req.headers();
    let cookie_str = "Cookie";
    let log_token = header_map.get(cookie_str);
    // check if there is a toke
    if log_token.is_some() {
        println!("{}", String::from("We are about to redirect"));
        // there is no cookie
        // Redirect to login
        //return Redirect::to("/login").permanent();
        return HttpResponse::Ok().status(StatusCode::from_u16(307).unwrap()).insert_header(("Location", "http://localhost:3000/login")).finish();
    }
    

    HttpResponse::Ok().body("You have started a party")
} // end of start_party