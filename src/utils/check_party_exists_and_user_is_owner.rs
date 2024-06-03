
use actix_web::{web::{Data}, HttpResponse};


use super::{convert_to_object_id::convert_to_object_id, response::JsonResponse};
use crate::models::party_model::*;

use crate::application_data::*;

/// Checks if a given user owns a party
pub async fn check_party_exists_and_user_is_owner_method(user_id: &str, app_data: Option<&Data<ApplicationData>>) -> (bool, HttpResponse) {
        // check that the user owns the party and that the party exists
        //let user_id = ObjectId::parse_str(user_id).expect("Should parse object id");
        let possible_user_id = convert_to_object_id(user_id);

        if possible_user_id.is_none() {
            // could not convert user_id into object id
            return (false, HttpResponse::Conflict().json(JsonResponse::new(false, false, String::from("Could not convert user id into object id"))));
        }

        // otherwise all good, check for a party

        let user_id = possible_user_id.unwrap();
        let party_collection = PartyCollection::new(app_data);
        let party = party_collection.query_by_owner(user_id).await;
        if party.is_none() {
            // there is no party
            return (false, 
                HttpResponse::Conflict().
                json(JsonResponse::new(false, false, String::from("User does not own a party"))));
        }

        // all gucci, 
        (true, HttpResponse::ImATeapot().finish())
}