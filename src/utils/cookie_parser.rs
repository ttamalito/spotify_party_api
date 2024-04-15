use std::collections::HashMap;
///
/// Struct to Represent a Cookie
#[derive(Debug)]
pub struct MyCookie {
    key: String,
    value: String
}

/// Implementation block of static functions
impl MyCookie {
    fn new(key: String, value: String) -> MyCookie {
        MyCookie {
            key: key,
            value: value
        }
    } // end of new
}

impl MyCookie {
    pub fn get_value_as_ref(&self) -> &String {
        &self.value
    }

    pub fn get_key_as_ref(&self) -> &String {
        &self.key
    } // end of getter

    pub fn get_key_as_str(&self) -> &str {
        self.key.as_str()
    }
}



pub fn parse_cookies(value:&str) -> HashMap<String, MyCookie> {
    //let mut result: Vec<MyCookie> = Vec::new();
    let mut result: HashMap<String, MyCookie> = HashMap::new();
    // get the length
    let len = value.len() - 1;
    println!("{}", len);
    let mut key_cookie = String::from("");
    let mut value_cookie = String::from("");
    let mut found_equal = false;
    // iterate through all the chars
    for (index, c) in value.char_indices() {
        println!("{} {} {} {}", String::from("index"), index, String::from("char"), c);
        // chekc if it is the space
        if c == ' ' {
            continue;
        }
        // check if it is the end of a key value pair
        if c == ';' {
            found_equal = false; // so that it starts from the key
            let key_clone = key_cookie.clone();
            //let str_key: &str = key_clone.as_str();
            let value_clone = value_cookie.clone();            
            let cookie = MyCookie::new(key_clone, value_clone);
            let key_str = key_cookie.clone();
            result.insert(key_str, cookie);
            key_cookie.clear();
            value_cookie.clear();
            continue;
        }

        if index == len {
            // we have reached the end, create a MyCookie
            value_cookie.push(c);
            let key_clone = key_cookie.clone();
            //let str_key = key_cookie.clone().as_str();
            let value_clone = value_cookie.clone(); 
            let key_str = key_clone.clone();        
            let cookie = MyCookie::new(key_clone, value_clone);
            result.insert(key_str, cookie);
            key_cookie.clear();
            value_cookie.clear();
            continue;
        }

        // check if there is an =
        if c == '=' {
            found_equal = true;
            continue;
        }
        // check if it should go to the key or the value
        if found_equal {
            // push it to the value
            value_cookie.push(c);
        } else {
            key_cookie.push(c);
        }
        
    } // end of for loop
    result
}