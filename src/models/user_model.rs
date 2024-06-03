
use actix_web::{web::Data};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::application_data::ApplicationData;
use crate::utils::collections_enum::*;

use mongodb::bson::{doc};
use mongodb::Collection;

pub struct User {
    collection: Collection<UserDocument>
}

impl User {
    /// Constructor to create a new user, with the respective collection
    /// #Arguments
    /// *data - The application data
    pub fn new(data: Option<&Data<ApplicationData>>) -> Self {
        let app_data = data.unwrap();
        let inner_data = app_data.get_ref();
        let database = inner_data.get_database();
        if database.is_none() {
            // does not have a database
            panic!("Could not access the database");
        }

        let collection: Collection<UserDocument> = database.unwrap().collection(get_collection(Collections::USERS).as_str());

        // create the new user
        User {
            collection: collection
        }
    } // ends new

    pub async fn query_user_by_email(&self, email: &str) -> Option<UserDocument> {
        // through the collection access the database
        let filter = doc! {"email": email};
        let result = self.collection.find_one(filter, None).await.expect("Could not retrive user from database");
        result
    } // end of query user by email

    /// Make the user the owner of a party
    pub async fn add_owned_party(&self, id: ObjectId, party_id: ObjectId) -> bool {
        let filter = doc! {"_id": id};

        let update = doc! {"$set": doc! {"owned_party": party_id}};
        // add the owned party
        let result = self.collection.update_one(filter, update, None).await.expect("Should update the document");
        if result.modified_count == 1 {
            // one was modified
            return true;
        }
        return false;
    }

    /// Removes the owned party from a user
    pub async fn remove_owned_party(&self, id: ObjectId) -> bool {
        let filter = doc! {"_id": id};

        let update = doc! {"$set": doc! {"owned_party": None::<ObjectId>}};
        let result = self.collection.update_one(filter, update, None).await.expect("Should remove owned party");
        // return
        result.modified_count == 1
    }

    pub async fn query_user_by_id(&self, id: ObjectId) -> Option<UserDocument> {
        let filter = doc! {"_id": id};
        let result = self.collection.find_one(filter, None).await.expect("Should query the user");
        result
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDocument {
    pub email: String,
    pub name: String,
    pub password: String,
    #[serde(skip_serializing)]
    pub _id: Option<ObjectId>,
    pub owned_party: Option<ObjectId>,
}