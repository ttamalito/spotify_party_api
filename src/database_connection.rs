use mongodb::{Collection, Database};
use mongodb::{Client, options::ClientOptions, error};
use mongodb::bson::{doc, Document};

/// Represents the Database client
pub struct DB {
    /// The actual database client
    client: Option<Client>,
    database: Option<Database>
}

impl DB {
    /// Constructor for the struct
    fn new() -> Self {
        DB {
            client: None,
            database: None
        }
    }

    /// Returns the Client as a reference in an Option 
    pub fn get_client(&self) -> Option<&Client> {
        if self.client.is_some() {
            // there is a value
            return self.client.as_ref();
        } else {
            Option::None
        }
    } // end of get_client

    /// Adds the client to the respective struct
    /// # Arguments
    /// * client - The client to add
    fn add_client(&mut self, client: Client) {
        // check if there is already a client
        if self.client.is_some() {
            // panic
            panic!("Trying to set 2 database clients");
        }

        // add the client
        self.client = Option::Some(client);
        // add the database
        self.database = Option::Some(self.client.as_ref().unwrap().database("spotify_party"));
    }

    /// Returns an Optional for the Database
    pub fn get_database(&self) -> Option<&Database> {
        if self.database.is_some() {
            return self.database.as_ref();
        } else {
            None
        }
    } // end of get_database

} // end of impl




/// Connects to the database
pub async fn connect_to_db(uri: &str) -> Result<Box<DB>, Box<dyn std::error::Error>> {
    
// Parse a connection string into an options struct.
//let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

// Get a handle to the deployment.
//let client = Client::with_options(client_options).unwrap();

let client = Client::with_uri_str("mongodb://localhost:27017").await?;

// create a new struct
let mut db = DB::new();
// add the client to the struct
db.add_client(client);

// put the db in the heap
let db_pointer = Box::new(db);

//let db_ref = &*db_pointer;

//let db_ref2 = *db_pointer;

// Get a handle to a database.
//let db = client.database("spotify_party");

// Get a handle to a collection in the database.
//let collection: Collection<Document> = db.collection("books");
//println!("{}", String::from("all gucci"));
/*
let docs = vec![
    doc! { "title": "1984", "author": "George Orwell" },
    doc! { "title": "Animal Farm", "author": "George Orwell" },
    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
]; */
//let to_insert =  doc! { "title": "1984", "author": "George Orwell" };
// Insert some documents into the "mydb.books" collection.
//let result = collection.insert_one(to_insert, None).await?;
//println!("{:?}", result);
println!("{}", String::from("Connection to the database established"));
Ok(db_pointer)
}