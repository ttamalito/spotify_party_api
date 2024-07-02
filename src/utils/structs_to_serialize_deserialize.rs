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