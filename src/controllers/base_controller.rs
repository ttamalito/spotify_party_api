use actix_web::{get, post, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::application_data::ApplicationData;

// import mongodb
use mongodb::bson::{doc, Document};

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