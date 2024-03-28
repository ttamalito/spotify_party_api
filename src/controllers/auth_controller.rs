use actix_web::{post, web::{self, Data}, HttpRequest, HttpResponse, Responder, http::{StatusCode}};
use serde::Deserialize;

use crate::application_data::ApplicationData;
use crate::utils::collections_enum::Collections;
use crate::utils::collections_enum::get_collection;


// mongodb
use mongodb::{bson::{doc, Document, to_document}, Collection};
use mongodb::bson;

/*
Struct to deserialize the URL Encoded data sent through the form
*/
#[derive(Deserialize)]
struct LoginData {
    email: String,
    password: String
}

#[post("/login")]
async fn post_login(form: web::Form<LoginData>) -> impl Responder {
    println!("Email: {}", form.email);
    println!("Password: {}", form.password);
    HttpResponse::Ok()
    .insert_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
    .body("Thank You for login in")
}


/*
Struct to deserialize the Form of signup
*/
#[derive(Deserialize)]
struct SignupData {
    name: String,
    email: String,
    password: String
}

/// Controller to handle the logic of a user signing up
#[post("/signup")]
async fn post_signup(req: HttpRequest, form: web::Form<SignupData>) -> impl Responder {
    let email = form.email.as_str();
    let name = form.name.as_str();
    let password = form.password.as_str();

    // here we would need to hash the password

    // now write the data
    let app_data = req.app_data::<Data<ApplicationData>>();

    // check if it is empty
    if app_data.is_none() {
        return HttpResponse::Ok().status(StatusCode::from_u16(401).unwrap()).body("Could not access the database, sorry!");
    }

    // get the collection
    let collection: Collection<Document> = app_data.unwrap().as_ref().get_database().unwrap().collection(get_collection(Collections::USERS).as_str());

    // create the data to add
    let to_add = doc! {"email": email, "name": name, "password": password};
    // insert it
    let result = collection.insert_one(to_add, None).await.expect("Could not add a doc to users collection");
    println!("{:?}", result);

    let inserted_id = bson::oid::ObjectId::to_hex(result.inserted_id.as_object_id().unwrap());
    let string_body = String::from("Thank you for signin up, this is your OBject Id:") + inserted_id.as_str();
    // send the response
    HttpResponse::Ok()
    .insert_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
    .body(string_body)
}
