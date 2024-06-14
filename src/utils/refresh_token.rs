use base64::prelude::*;

use crate::{controllers::party_controller::AccessToken, models::party_model::{Party, PartyAccessToken, PartyCollection}};
use awc::{Client, Connector};
use mongodb::bson::oid::ObjectId;
use openssl::ssl::{SslConnector, SslMethod};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};
/// Refreshes a token
/// * token - The Refresh Token saved in the database
/// * code - The code that was received when the first access token was requested
async fn refresh_token(
    token: String,
    client_id: &str,
    client_secret: &str,
    user_id: ObjectId,
    party_collection: PartyCollection,
) -> bool {
    // get the party from the database
    let party = party_collection.query_by_owner(user_id).await;

    if party.is_none() {
        return false; // the user does not own a party
    }

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
    let party = party.unwrap();
    let party_id = party._id;
    let party_access_token_struct = PartyAccessToken {
        access_token: payload.access_token,
        token_type: payload.token_type,
        expires_in: payload.expires_in,
        refresh_token: payload.refresh_token,
        scope: payload.scope
    };
    party_collection.set_access_token_data(party_id, party_access_token_struct).await;

    true
}
