
use actix_web::{web::Data};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::application_data::ApplicationData;
use crate::utils::collections_enum::*;

use mongodb::bson::{doc, Document};
use mongodb::Collection;

pub struct PartyCollection {
    collection: Collection<Party>
}

impl PartyCollection {
    pub fn new(data: Option<&Data<ApplicationData>>) -> Self {
        let app_data = data.unwrap();
        let inner_data = app_data.get_ref();
        let database = inner_data.get_database();
        if database.is_none() {
            // does not have a database
            panic!("Could not access the database");
        }

        let collection: Collection<Party> = database.unwrap().collection(get_collection(Collections::PARTIES).as_str());

        // create the party
        PartyCollection {
            collection: collection
        }
    } // ends new

    /// Queries a document by id
    pub async fn query_by_id(&self, id: ObjectId) -> mongodb::error::Result<Option<Party>> {
        let filter = doc! {"_id": id};
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// inserts a memeber to the party asynchrounouslys
    pub async fn insert_member(&self, id: ObjectId, member: ObjectId) -> bool {
        let filter = doc! {"_id": id};
        let update = doc! {"$push": doc! {"members": member}};
        let result = self.collection.update_one(filter, update, None).await.expect("Should update the document");
        if result.modified_count == 1 {
            return true;
        } else {
            return false;
        }
    }

    /// saves a party to the database
    pub async fn save_party(&self, party: Party) -> ObjectId {
        let result = self.collection.insert_one(party, None).await.expect("Should insert a new party doc");
        result.inserted_id.as_object_id().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Party {
    pub _id: Option<ObjectId>,
    pub members: Vec<ObjectId>,
    pub owner: ObjectId,
    pub access_token: PartyAccessToken
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PartyAccessToken {
    access_token: String,
    token_type: String,
    expires_in: i32
}