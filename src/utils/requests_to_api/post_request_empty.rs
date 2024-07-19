
use actix_web::{http::{StatusCode}};

use awc::{Connector, Client};
use openssl::ssl::{SslConnector, SslMethod};
use hmac::{Mac};



use std::{time::Duration};
use super::errors_spotify::*;

/// Sends a post request to the api with the given authorization header
/// Returns wheter the operation was succsessful and the response status
pub async fn post_request_emtpy_body(auth_header: &str, url: &str) -> (bool, StatusCode) {
            // send the request to the api
            let builder = SslConnector::builder(SslMethod::tls()).unwrap();

            let client = Client::builder().timeout(Duration::from_secs(50)) // if you get Timeout add  to the client, see party_controller.request_token 134
                .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(54)))
                .finish();
        
            //println!("{}", req_body);
            let mut response = client.post(url).timeout(Duration::from_secs(45)).
            insert_header(("Authorization", auth_header))
            .send().await.unwrap();
            //println!("{:?}", response.headers());
            // check the response code
            //println!("{:?}", response.version());
            println!("{:?}", response.status());

            if response.status() == StatusCode::NO_CONTENT {
                // all good
                return (true, response.status());
            }
            if response.status() == StatusCode::OK {
                return (true, response.status());
            }

                
    //let payload = response.json::<serde_json::Value>().await.expect("What ever");
    let payload = response.json::<MainError>().await.expect("Should deserialize");
    println!("{:?}", payload);
    (false, response.status())

}