use actix_web::{web::Data, HttpResponse};

use super::{convert_to_object_id::convert_to_object_id, response::JsonResponse};
use crate::models::party_model::*;

use crate::application_data::*;

/// Checks if a given user owns a party
/// Returns a tuple with the following values:
/// - A boolean indicating whether the user owns a party or not.
/// - An HttpResponse with the appropriate status code and message.
/// - An optional string containing the access token if the user owns a party.
/// - An optional string containing the party id if the user owns a party.
pub async fn check_party_exists_and_user_is_owner_method(
    user_id: &str,
    app_data: Option<&Data<ApplicationData>>,
) -> (bool, HttpResponse, Option<String>, Option<String>) {
    // check that the user owns the party and that the party exists
    //let user_id = ObjectId::parse_str(user_id).expect("Should parse object id");
    let possible_user_id = convert_to_object_id(user_id);

    if possible_user_id.is_none() {
        // could not convert user_id into object id
        return (
            false,
            HttpResponse::Conflict().json(JsonResponse::new(
                false,
                false,
                String::from("Could not convert user id into object id"),
            )),
            None,
            None
        );
    }

    // otherwise all good, check for a party

    let user_id = possible_user_id.unwrap();
    let party_collection = PartyCollection::new(app_data);
    let party = party_collection.query_by_owner(user_id).await;
    if party.is_none() {
        // there is no party
        return (
            false,
            HttpResponse::Conflict().json(JsonResponse::new(
                false,
                false,
                String::from("User does not own a party"),
            )),
            None,
            None
        );
    }
    // get the data from the party
    let party = party.unwrap();
    let access_token = party.get_access_token();
    let party_id = party._id.to_hex();

    // all gucci,
    (true, HttpResponse::ImATeapot().finish(), Some(access_token), Some(party_id))
}
