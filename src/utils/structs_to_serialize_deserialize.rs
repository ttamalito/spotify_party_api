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

/// Response of the api when requesting the playback state and it is a track currently playing
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePlaybackStateTrack {
    pub response_object: MainResponsePlaybackStateTrackObjectItem,
}

/// Response of the api when requesting the playback state and it is an episode currently playing
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePlaybackStateEpisode {
    pub response_object: MainResponsePlaybackStateEpisodeObjectItem,
}


/// Response object that contains all the data about the playback state
#[derive(Serialize, Deserialize, Debug)]
pub struct MainResponsePlaybackStateTrackObjectItem {
    pub device: Device,
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    pub timestamp: i64,
    pub progress_ms: Option<i64>,
    pub is_playing: bool,
    pub item: TrackObject,
    pub currently_playing_type: String,
    #[serde(default)]
    pub actions: Actions

}

/// Device object inside the MainResponsePlaybackState
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    id: Option<String>,
    is_active: bool,
    is_private_session: bool,
    is_restricted: bool,
    name: String,
    #[serde(rename = "type")]
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
pub struct TrackObject {
    album: Album,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    disc_number: i32,
    duration_ms: i32,
    explicit: bool,
    external_ids: ExternalIDs,
    external_urls: ExternalURLs,
    href: String,
    id: String,
    is_playable: Option<bool>,
    linked_from: Option<LinkedFrom>,
    #[serde(default)]
    restrictions: Restrictions,
    name: String,
    popularity: i32,
    preview_url: Option<String>,
    track_number: i32,
    #[serde(rename = "type")]
    item_type: String,
    uri: String,
    is_local: bool
}

/// Actions Object inside the MainResponsePlaybackState
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Actions {
    interrupting_playback: Option<bool>,
    pausing: Option<bool>,
    resuming: Option<bool>,
    seeking: Option<bool>,
    skipping_next: Option<bool>,
    skipping_prev: Option<bool>,
    toggling_repeat_context: Option<bool>,
    toggling_shuffle: Option<bool>,
    toggling_repeat_track: Option<bool>,
    transferring_playback: Option<bool>
}


/// Album Object inside the TrackObject
#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    album_type: String,
    total_tracks: i32,
    available_markets: Vec<String>,
    external_urls: ExternalURLs,
    href: String,
    id: String,
    images: Vec<Image>,
    name: String,
    release_date: String,
    release_date_precision: String,
    #[serde(default)]
    restrictions: Restrictions,
    #[serde(rename = "type")]
    album_type_fixed: String, // this will alway be "album"
    uri: String,
    artists: Vec<Artist>
}

/// Artist Object inside the Album and TrackObject
#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    external_urls: ExternalURLs,
    href: String,
    id: String,
    name: String,
    #[serde(rename = "type")]
    artist_type: String,
    uri: String
}

/// ExternalIDs Object inside the TrackObject
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalIDs {
    isrc: String,
    #[serde(default)]
    ean: String,
    #[serde(default)]
    upc: String
}

/// Image Object inside the Album
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    height: i32,
    url: String,
    width: i32
}

/// LinkedFrom Object inside the TrackObject
#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedFrom {

}

/// Restrictions Object inside the Album and TrackObject
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Restrictions {
    reason: String
}

/// Struct to represent an episode object
#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeObject {
    audio_preview_url: Option<String>,
    description: String,
    html_description: String,
    duration_ms: i32,
    explicit: bool,
    external_urls: ExternalURLs,
    href: String,
    id: String,
    images: Vec<Image>,
    is_externally_hosted: bool,
    is_playable: bool,
    #[serde(default)]
    language: String,
    languages: Vec<String>,
    name: String,
    release_date: String,
    release_date_precision: String,
    resume_point: ResumePoint,
    #[serde(rename = "type")]
    type_fixed: String,
    uri: String,
    #[serde(default)]
    restrictions: Restrictions,
    show: Show
}

/// ResumePoint Object inside the EpisodeObject
#[derive(Serialize, Deserialize, Debug)]
pub struct ResumePoint {
    fully_played: bool,
    resume_position_ms: i32
}

/// Show Object inside the EpisodeObject
#[derive(Serialize, Deserialize, Debug)]
pub struct Show {
    available_markets: Vec<String>,
    copyrights: Vec<Copyright>,
    description: String,
    html_description: String,
    explicit: bool,
    external_urls: ExternalURLs,
    href: String,
    id: String,
    images: Vec<Image>,
    is_externally_hosted: bool,
    is_playable: bool,
    languages: Vec<String>,
    media_type: String,
    name: String,
    publisher: String,
    #[serde(rename = "type")]
    type_fixed: String,
    uri: String,
    total_episodes: i32
}

/// Copyright Object
#[derive(Serialize, Deserialize, Debug)]
pub struct Copyright {
    text: String,
    #[serde(rename = "type")]
    copy_right_type: String
}


/// Response struct for Episode Object case
#[derive(Serialize, Deserialize, Debug)]
pub struct MainResponsePlaybackStateEpisodeObjectItem {
    device: Device,
    repeat_state: String,
    shuffle_state: bool,
    context: Option<Context>,
    timestamp: i64,
    progress_ms: Option<i64>,
    is_playing: bool,
    item: EpisodeObject,
    currently_playing_type: String,
    #[serde(default)]
    actions: Actions
}