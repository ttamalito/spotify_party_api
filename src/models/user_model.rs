
use actix_web::{web::Data};
use crate::application_data::ApplicationData;
use crate::utils::collections_enum::*;

use mongodb::bson::{doc, Document};
use mongodb::Collection;

pub struct User {
    collection: Collection<Document>
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

        let collection: Collection<Document> = database.unwrap().collection(get_collection(Collections::USERS).as_str());

        // create the new user
        User {
            collection: collection
        }
    } // ends new

    pub async fn query_user_by_email(&self, email: &str) -> Option<Document> {
        // through the collection access the database
        let filter = doc! {"email": email};
        let result = self.collection.find_one(filter, None).await.expect("Could not retrive user from database");
        result
    } // end of query user by email
}