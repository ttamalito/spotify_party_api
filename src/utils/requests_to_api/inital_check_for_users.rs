
use actix_web::{web::{Data}, HttpRequest, HttpResponse};

use super::super::check_login::check_login;
use super::super::response::JsonResponse;
use super::super::build_headers::build_authorization_header::get_authorization_header;
use super::super::check_party_exists_and_user_is_owner::check_party_exists_and_user_is_owner_method;
use super::super::super::application_data::ApplicationData;

/// Checks that the user making the request is the owner of the party and that he has an authorization token
pub async fn intial_checkup(req: HttpRequest) -> (HttpResponse, Option<String>) {
    let (logged, user_id) = check_login(req.headers());

    if !logged {
        // not logged in
        return (HttpResponse::Unauthorized().json(JsonResponse::redirect_to_login()), None);
    }

    // check that the user is the owner of the party
    let (is_owner, response, possible_access_token, _possible_party_id) = check_party_exists_and_user_is_owner_method(&user_id, req.app_data::<Data<ApplicationData>>()).await;
    if !is_owner {
        return (response, None); // if not the owenr send the corresponding response
    }

    // get the authorization header
    let (exists_token, auth_header) = get_authorization_header(req.headers(), possible_access_token);
    println!("There is a token: {} line 27 initial_check_for_users.rs", exists_token);
    if !exists_token {
        // there is no token
        return (HttpResponse::Unauthorized().
        json(JsonResponse::new(false, 
            true, String::from("http://localhost:3000/startParty"))), None);  // TODO --- change the url to redirect, to refresh the token
    }


    // all good
    return (HttpResponse::Ok().finish(), Some(auth_header));
}