use actix_web::{post, web::{self, Data}, HttpRequest, HttpResponse, Responder, http::{StatusCode}};
use serde::Deserialize;

use crate::{application_data::ApplicationData, models::user_model::User};
use crate::utils::collections_enum::Collections;
use crate::utils::collections_enum::get_collection;
use crate::models::user_model::{ UserDocument};
use crate::utils::response::*;


use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

// mongodb
use mongodb::{bson::{doc, Document}, Collection};
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
async fn post_login(req: HttpRequest, form: web::Form<LoginData>) -> impl Responder {
    let email = form.email.as_str();
    let _password = form.password.as_str();

    // access the database
    let app_data = req.app_data::<Data<ApplicationData>>();
    if app_data.is_none() {
        // there is no application data
        println!("{}", String::from("There is no application data in POST /login"));
        return HttpResponse::Ok().insert_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
        .status(StatusCode::from_u16(401).unwrap())
        .body("Something went wrong in our side.");
    }
    // get the collection
    let collection: Collection<Document> = app_data.unwrap().as_ref().get_database().unwrap().collection(get_collection(Collections::USERS).as_str());
    // now find the corresponding entry
    let filter = doc! {"email": email};
    //println!("{}", email);
    let result = collection.find_one(filter, None).await;
    let serialized_doc = result.as_ref().unwrap();
    let user_struct: UserDocument = bson::from_bson(bson::Bson::Document(serialized_doc.as_ref().unwrap().to_owned())).expect("Could not deserialize it");
    //println!("{:?}", &user_struct);
    let what = result.as_ref();
    let que = what.unwrap();
    if result.is_ok() {
        if (*que).is_some(){
            // we have a result
            println!("{}", result.as_ref().unwrap().as_ref().unwrap());
        
        } else {
            println!("{}", String::from("There is no document"));
        }
    } else {
        println!("{}", String::from("There was an error trying to retrieve the results"))
    }

    // generate the token
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").expect("could not generate secret key!");
    let mut claims = BTreeMap::new();
    claims.insert("sub", "user-id!!!");
    let user_id = user_struct._id.unwrap().to_string();
    claims.insert("id", user_id.as_str());
    let token_str = String::from("jwt=") + claims.sign_with_key(&key).expect("COuld not sign token").as_str();
    HttpResponse::Ok()
    //.insert_header(("login", token_str.as_str()))
    .body(token_str)
} // end of post /login


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

    // check if there is a user with that email already
    let data_to_pass = req.app_data::<Data<ApplicationData>>();
    if data_to_pass.is_none() {
        return HttpResponse::Ok().status(StatusCode::from_u16(401).unwrap()).body("Could not access the database, sorry!");
    }
    let user = User::new(data_to_pass);

    // check if there is a user with that email
    let some = user.query_user_by_email(email).await;
    if some.is_some() {
        // there is someone with that email
        return HttpResponse::Ok().status(StatusCode::from_u16(401).unwrap()).body("There is a user with that email already!");
    }

    // here we would need to hash the password

    // now write the data to the data base
    let app_data = req.app_data::<Data<ApplicationData>>();

    // check if it is empty
    if app_data.is_none() {
        return HttpResponse::Ok().status(StatusCode::from_u16(401).unwrap()).body("Could not access the database, sorry!");
    }

    // get the collection
    let collection: Collection<Document> = app_data.unwrap().as_ref().get_database().unwrap().collection(get_collection(Collections::USERS).as_str());

    
    // create the data to add
    //let to_add = doc! {"email": email, "name": name, "password": password};
    let to_add_doc = UserDocument {
        email: email.to_owned(),
        password: password.to_owned(),
        name: name.to_owned(),
        _id: None,
        owned_party: None

    };
    let serialized_doc = bson::to_bson(&to_add_doc).expect("Could not serialize the document in sign up");
    let to_add = serialized_doc.as_document().unwrap().to_owned();
    // insert it
    let result = collection.insert_one(to_add, None).await.expect("Could not add a doc to users collection");
    println!("{:?}", result);

    let inserted_id = bson::oid::ObjectId::to_hex(result.inserted_id.as_object_id().unwrap());
    let _string_body = String::from("Thank you for signin up, this is your OBject Id:") + inserted_id.as_str();
    // send the response, with the proper json
    let response = JsonResponseForSigningUp::redirect_to_login(String::from("jdjd"), inserted_id);
    HttpResponse::Ok()
    .insert_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
    .json(response)
}
