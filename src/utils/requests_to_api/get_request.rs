use actix_web::http::StatusCode;

use awc::{Client, Connector};
use hmac::Mac;
use openssl::ssl::{SslConnector, SslMethod};

use super::errors_spotify::*;
use std::time::Duration;
use crate::utils::structs_to_serialize_deserialize::{MainResponsePlaybackStateEpisodeObjectItem, MainResponsePlaybackStateTrackObjectItem};

pub async fn get_request_get_play_back_state(auth_header: &str, url: &str) -> (bool, StatusCode, Option<MainResponsePlaybackStateTrackObjectItem>, Option<MainResponsePlaybackStateEpisodeObjectItem>) {
    // send the request to the api
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .timeout(Duration::from_secs(50)) // if you get Timeout add  to the client, see party_controller.request_token 134
        .connector(
            Connector::new()
                .openssl(builder.build())
                .timeout(Duration::from_secs(54)),
        )
        .finish();

    //println!("{}", req_body);
    let mut response = client
        .get(url)
        .timeout(Duration::from_secs(45))
        .insert_header(("Authorization", auth_header))
        .send()
        .await
        .unwrap();
    //println!("{:?}", response.headers());
    // check the response code
    //println!("{:?}", response.version());
    println!("Response Code from GET Request:{:?}", response.status());

    // if the response is 200, deserialize it
    if response.status() == StatusCode::OK {
        let state = response
            .json::<MainResponsePlaybackStateTrackObjectItem>()
            .await;
    
        // if this could not be deserialized, try to deserialize it as an episode
        if state.is_err() {
            let state_episode = response
                .json::<MainResponsePlaybackStateEpisodeObjectItem>()
                .await;
            if state_episode.is_err() {
                // throw an errors
                //panic!("Could not deserialize response from GET REquest");
                println!("{}", String::from("User current playback state is a podcast, it is not supported at the moment"));
                return (false, response.status(), None, None);
            } // else it is an epidose
            else {
                let state_episode = state_episode.unwrap();
                return (true, response.status(), None, Some(state_episode));
            }
        } else {
            // it is a track
            let state = state.unwrap();
            return (true, response.status(), Some(state), None);
        }
    } // Status === 200

    if response.status() == StatusCode::NO_CONTENT {
        return (true, response.status(), None, None);
        
    }
    // it was an error
    let payload = response
        .json::<MainError>()
        .await
        .expect("Should deserialize");
    println!("{:?}", payload);
    (false, response.status(), None, None)
}
