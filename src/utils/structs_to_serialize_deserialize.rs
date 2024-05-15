use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ResponseForQueueToJoinParty {
    pub result: bool,
    pub message: String,
    pub users: Vec<ObjectId>
}