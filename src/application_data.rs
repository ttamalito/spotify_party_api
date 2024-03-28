

use crate::database_connection::DB;
use mongodb::{Client, Database};
/// 
pub struct ApplicationData {
    database_box: Box<DB>
}

impl ApplicationData {
    pub fn new(database_box: Box<DB>) -> Self {
        ApplicationData { database_box }
    }

    pub fn get_database(&self) -> Option<&Database> {
        let what = self.database_box.as_ref();
        let opt_database = what.get_database();
        opt_database
    }

}