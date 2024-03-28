use mongodb::Collection;
use mongodb::{Client, options::ClientOptions, error};
use mongodb::bson::{doc, Document};
pub struct DB {
    client: Option<Client>
}

pub async fn connect_to_db(uri: &str) -> Result<Client, Box<dyn std::error::Error>> {
    
// Parse a connection string into an options struct.
//let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

// Get a handle to the deployment.
//let client = Client::with_options(client_options).unwrap();

let client = Client::with_uri_str("mongodb://localhost:27017").await?;
println!("{}", String::from("all good"));
// Get a handle to a database.
let db = client.database("spotify_party");

// Get a handle to a collection in the database.
let collection: Collection<Document> = db.collection("books");
println!("{}", String::from("all gucci"));
let docs = vec![
    doc! { "title": "1984", "author": "George Orwell" },
    doc! { "title": "Animal Farm", "author": "George Orwell" },
    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
];
let to_insert =  doc! { "title": "1984", "author": "George Orwell" };
// Insert some documents into the "mydb.books" collection.
let result = collection.insert_one(to_insert, None).await?;
println!("{:?}", result);
println!("{}", String::from("I was executed!"));
Ok(client)
}