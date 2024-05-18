use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorSpotify {
    //#[serde(skip_deserializing)]
    status: i32,
    message: String
}
/// Struct for deserializing Error From Spotify API
#[derive(Deserialize, Serialize, Debug)]
pub struct MainError {
    error: ErrorSpotify
}