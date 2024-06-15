
use actix_web::{http::{StatusCode}};

use awc::{Connector, Client};
use openssl::ssl::{SslConnector, SslMethod};
use hmac::{Mac};



use std::{time::Duration};
use super::errors_spotify::*;


pub async fn put_request_emtpy_body(auth_header: &str, url: &str) -> bool {
            // send the request to the api
            let builder = SslConnector::builder(SslMethod::tls()).unwrap();

            let client = Client::builder().timeout(Duration::from_secs(50)) // if you get Timeout add .timeout(Duration::from_secs(50)) to the client, see party_controller.request_token 134s
                .connector(Connector::new().openssl(builder.build()).timeout(Duration::from_secs(54)))
                .finish();
        
            //println!("{}", req_body);
            let mut response = client.put(url).timeout(Duration::from_secs(45)).
            insert_header(("Authorization", auth_header))
            .send().await;
            if response.is_err() {
                println!("{}", String::from("Timeout in put request"));
                return false;
            }
            let mut response = response.unwrap();
            //println!("{:?}", response.headers());
            // check the response code
            //println!("{:?}", response.version());
            println!("{:?}", response.status());

            if response.status() == StatusCode::NO_CONTENT {
                // all good
                return true;
            }

                
    //let payload = response.json::<serde_json::Value>().await.expect("What ever");
    let payload = response.json::<MainError>().await.expect("Should deserialize");
    println!("{:?}", payload);
    false

}