
use mongodb::bson::oid::ObjectId;

pub fn convert_to_object_id(id: &str) -> Option<ObjectId> {
    let possible = ObjectId::parse_str(id);
    if possible.is_err() {
        // we have an error
        return None;
    }

    // else it could be converted
    return Some(possible.unwrap());
} // end of function