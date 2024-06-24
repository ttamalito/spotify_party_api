use actix_web::{get, web::{Data}, HttpRequest, HttpResponse, Responder};
use crate::application_data::ApplicationData;
use crate::utils::check_login::check_login;
use crate::utils::structs_to_serialize_deserialize::ResponseUserHasParty;
use crate::models::party_model::*;
use mongodb::bson::oid::ObjectId;
use crate::utils::check_party_exists_and_user_is_owner::check_party_exists_and_user_is_owner_method;

// controller for the main page
#[get("/")]
async fn hello() -> impl Responder {
    
    HttpResponse::Ok().body("Hello world!- Using Rust!- And cargo watch")
    
}

#[get("/response")]
async fn foo(req: HttpRequest) -> impl Responder {
    // print the request
    //println!("The path: {:?}", req.path());
    //println!("The Req method: {:?}", req.method());
    //println!("The HTTP version: {:?}", req.version());
    println!("The headers: {:?}", req.headers());
    //println!("The app Config: {:?}", req.app_config());
   // println!("The app data: {:?}", req.app_data::<Data<T>>());
   // access the data
   let data = req.app_data::<Data<ApplicationData>>();
    if data.is_none() {
        println!("{}", String::from("WTF?? how is this happening?"));
        panic!("Putos");
    }
    let data = data.unwrap().as_ref();
   // get the database
   let database = data.get_database();
   if database.is_some() {
    // we have a database
    // let collection = database.unwrap().collection("tesing");
    // let to_insert =  doc! { "title": "PUTO", "author": "George Orwell" };
    // Insert some documents into the "mydb.books" collection.
    //let result = collection.insert_one(to_insert, None).await.expect("Verga no funciono esta mierdaa!");
    //println!("{:?}", result);
   }
   let header_map = req.headers();
   let cookie_str = "Cookie";
   let log_token = header_map.get(cookie_str);
   if log_token.is_some() {
    println!("{}", String::from("You have sent the cookie"));
   }
    HttpResponse::Ok().body("Te amo")
}
/// This function checks if the user is logged in and if they have a party.
///
/// # Arguments
///
/// * `req` - An instance of HttpRequest, used to access headers.
///
/// # Returns
///
/// * HttpResponse - Returns a JSON response with a boolean indicating if the user has a party and the party id if it exists.
#[get("/userHasParty")]
async fn user_has_party(req: HttpRequest) -> impl Responder {
    // see if the user is logged in
    let (logged, user_id) = check_login(req.headers());
    if !logged {
        return HttpResponse::Ok().json(ResponseUserHasParty {
            result: false,
            party_id: String::from("")
        });
    }
    // see if the user has a party
    let (owns_party, response, _access_token, party_id) = check_party_exists_and_user_is_owner_method(user_id.as_str(), req.app_data::<Data<ApplicationData>>()).await;

    if !owns_party {
        return response;
    }

    let party_id = party_id.unwrap();
    HttpResponse::Ok().json(ResponseUserHasParty {
        result: true,
        party_id
    })
}