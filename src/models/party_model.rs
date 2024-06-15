
use actix_web::{web::Data};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::application_data::ApplicationData;
use crate::utils::collections_enum::*;

use mongodb::bson::{doc};
use mongodb::Collection;

pub struct PartyCollection {
    collection: Collection<Party>
}
/// Implementation block for all the operations to be performed on the collection
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
    } // end of insert memeber

    /// saves a party to the database
    pub async fn save_party(&self, party: Party) -> ObjectId {
        let result = self.collection.insert_one(party, None).await.expect("Should insert a new party doc");
        result.inserted_id.as_object_id().unwrap()
    } // end of save party

    /// Queries a Party Document given the owner
    pub async fn query_by_owner(&self, owner_id: ObjectId) -> Option<Party> {
        let filter = doc! {"owner": owner_id};
        let result = self.collection.find_one(filter, None).await.expect("Should query the Party document");
        result
    } // end of query by owner

    /// Deletes a party from the database
    pub async fn delete_party(&self, id: ObjectId) -> bool {
        let filter = doc! {"_id": id};
        let result = self.collection.delete_one(filter, None).await.expect("Should delete party from the database");
        result.deleted_count == 1
    }

    /// inserts an object id to the people that have requested to join the party
    pub async fn insert_requested_to_join(&self, party_id: ObjectId, future_memeber: ObjectId) -> bool {
        let filter = doc! {"_id": party_id};
        let update = doc! {"$push": doc! {"requested_to_join": future_memeber}};
        let result = self.collection.update_one(filter, update, None).await.expect("Should update the document");
        if result.modified_count == 1 {
            return true;
        } else {
            return false;
        }
    } // end of insert_requested_to_join

    /// Removes a user from the requested_to_join queue
    pub async fn remove_user_from_queue(&self, party_id: ObjectId, user_id: ObjectId) -> bool {
        let filter = doc! {"_id": party_id};
        let update = doc! {"$pull": doc! {"requested_to_join": user_id}};
        let result = self.collection.update_one(filter, update, None).await.expect("Should update the document");
        if result.modified_count == 1 {
            return true;
        } 
        false
    }

    /// Set a new access token and all its parts
    pub async fn set_access_token_data(&self, party_id: ObjectId, data: PartyAccessToken) -> bool {
        let filter = doc! {"_id": party_id};
        let data_as_object = doc! {    "access_token": data.access_token,
            "token_type": data.token_type,
            "expires_in": data.expires_in,
            "refresh_token": data.refresh_token,
            "scope": data.scope};
        let data_as_doc = doc! {"access_token": data_as_object};
        let update = doc! {"$set" : data_as_doc};
        let result = self.collection.update_one(filter, update, None).await.expect("Should update Document");
        if result.modified_count == 1 {
            return true;    
        }
        // else, it was not successful
        false
    } // end of set_access_token_data
} // methods for PartyCollection

/// Struct to represent a party
#[derive(Deserialize, Serialize, Debug)]
pub struct Party {
    #[serde(skip_serializing)]
    pub _id: ObjectId,
    pub members: Vec<ObjectId>,
    pub owner: ObjectId,
    pub access_token: PartyAccessToken,
    pub requested_to_join: Vec<ObjectId>
}

/// Struct to represent an access token
#[derive(Deserialize, Serialize, Debug)]
pub struct PartyAccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String
}


impl Party {
    /// Constructor to create a completely new party (without ObjectId)
    pub fn new(owner: String, access_token: String, token_type: String, expires_in: i32, refresh_token: String, scope: String) -> Self {
        // convert the string to object id
        let object_id = ObjectId::parse_str(owner).expect("Should convert string to object id");
        Party {
            _id: ObjectId::new(),
            members: vec![],
            owner: object_id,
            access_token: PartyAccessToken {access_token, token_type, expires_in, refresh_token, scope},
            requested_to_join: vec![]
        }
    } // end of new

    pub fn get_members_as_ref(&self) -> &Vec<ObjectId> {
        return &self.members;
    }

    /// Retunrs the array of the users that have requested to join the party
    pub fn get_requested_to_join_as_ref(&self) -> &Vec<ObjectId> {
        return &self.requested_to_join;
    }

    /// Retreives the access token from a Party struct
    pub fn get_access_token(&self) -> String {
        return self.access_token.access_token.clone();
    }
}
