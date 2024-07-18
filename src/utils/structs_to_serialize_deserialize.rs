use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct ResponseForQueueToJoinParty {
    pub result: bool,
    pub message: String,
    pub users: Vec<ObjectId>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUserHasParty {
    pub result: bool,
    pub party_id: String
}

/// Response of the api when requesting the playback state
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePlaybackState {
    pub response_object: MainResponsePlaybackState
}

/// Response object that contains all the data about the playback state
#[derive(Serialize, Deserialize, Debug)]
pub struct MainResponsePlaybackState {
    pub device: Device,
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    pub timestamp: i64,
    pub progress_ms: Option<i64>,
    pub is_playing: bool,
    pub item: Option<Item>,
    pub current_track: Option<CurrentTrack>,
    pub repeat_context: Option<RepeatContext>,

}

/// Device object inside the MainResponsePlaybackState
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    id: Option<String>,
    is_active: bool,
    is_private_session: bool,
    is_restricted: bool,
    name: String,
    type_: String,
    volume_percent: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalURLs {
    spotify: String
}


/// Context Object inside the MainResponsePlaybackState
#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    #[serde(rename = "type")]
    context_type: String,
    href: String,
    external_urls: ExternalURLs,
    uri: String
}


/// Item Object inside the MainResponsePlaybackState
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    album: Album,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    duration_ms: i32,
    explicit: bool,
    href: String,
    id: String,
    is_playable: bool,
    linked_from: Option<LinkedFrom>,
    name: String,
    popularity: i32,
    preview_url: Option<String>,
    track_number: i32,
    type_: String,
    uri: String
}

