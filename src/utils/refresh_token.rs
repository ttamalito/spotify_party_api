use actix_web::{web::Data, HttpRequest};
use base64::prelude::*;

use crate::{application_data::ApplicationData, controllers::party_controller::AccessToken, models::party_model::{Party, PartyAccessToken, PartyCollection}};
use awc::{Client, Connector};
use mongodb::bson::oid::ObjectId;
use openssl::ssl::{SslConnector, SslMethod};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};
use crate::utils::check_login::check_login;
use crate::constants::*;

/// Refreshes a token
/// By Getting all the necessary data from the database
pub async fn refresh_token(
    req: HttpRequest
) -> bool {
    // get the party from the database
    let party_collection = PartyCollection::new(req.app_data::<Data<ApplicationData>>());
    // get the user id from the cookies
    let (logged_id, user_id) = check_login(req.headers());
    // convert the user_id to an object id
    let user_id = ObjectId::parse_str(user_id).ok().unwrap();
    let party = party_collection.query_by_owner(user_id).await;

    if party.is_none() {
        return false; // the user does not own a party
    }

    let party = party.unwrap();
    let token = party.access_token.refresh_token;

    let client_id = CLIENT_ID;
    let client_secret = CLIENT_SECRET;

    // create the request body
    let req_body = format!(
        "{}{}{}",
        "grant_type=refresh_token", "&refresh_token=", token
    );
    // authorization header
    let authorization_header = format!("{}{}{}", client_id, ":", client_secret);

    let base64_authorization_header = format!(
        "{}{}",
        "Basic ",
        BASE64_STANDARD.encode(authorization_header)
    );

    // send the request
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .timeout(Duration::from_secs(50))
        .connector(
            Connector::new()
                .openssl(builder.build())
                .timeout(Duration::from_secs(50)),
        )
        .finish();

    let mut response = client
        .post("https://accounts.spotify.com/api/token")
        .timeout(Duration::from_secs(50))
        .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
        .insert_header(("Authorization", base64_authorization_header))
        .send_body(req_body)
        .await
        .unwrap();

    println!("{:?}", response.version());
    println!("{:?}", response.status());
    let payload = response.json::<AccessToken>().await.unwrap();
    println!("Access token{}", payload.access_token);
    println!("{}", payload.token_type);
    println!("{}", payload.expires_in);
    println!("Scope: {}", payload.scope);
    println!("Refresh TOken: {}", payload.refresh_token);

    // add the new token to the database
    let party_id = party._id;
    let party_access_token_struct = PartyAccessToken {
        access_token: payload.access_token,
        token_type: payload.token_type,
        expires_in: payload.expires_in,
        refresh_token: payload.refresh_token,
        scope: payload.scope
    };
    let insertion_result = party_collection.set_access_token_data(party_id, party_access_token_struct).await;

    // true or false
    insertion_result
}
