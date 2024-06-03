use serde::{Deserialize, Serialize};
use actix_web::{get, http::{header::{ContentType, HeaderValue}, StatusCode}, post, put, web::{self, Data, Json, Redirect}, HttpRequest, HttpResponse, Responder};
use jwt::Store;
use awc::{Connector, Client};
use openssl::ssl::{SslConnector, SslMethod};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use mongodb::bson::oid::ObjectId;
use std::{cmp::Ordering, collections::BTreeMap, str::FromStr, time::Duration};
use super::errors_spotify::*;


pub async fn put_request_emtpy_body(auth_header: &str, url: &str) -> bool {
            // send the request to the api
            let builder = SslConnector::builder(SslMethod::tls()).unwrap();

            let client = Client::builder() // if you get Timeout add .timeout(Duration::from_secs(50)) to the client, see party_controller.request_token 138
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