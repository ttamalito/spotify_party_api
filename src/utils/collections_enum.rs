
/// Contains all the collections for the database
pub enum Collections {
    USERS,
    TESTING,
    PARTIES
}

/// Returns the associated string with a given collection
pub fn get_collection(collection: Collections) -> String {
    let result = match collection {
        Collections::USERS => {String::from("users")},
        Collections::TESTING => {String::from("testing")},
        Collections::PARTIES => {String::from("parties")}
    };
    result
}