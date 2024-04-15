

pub struct MyCookie {
    key: String,
    value: String
}

impl MyCookie {
    fn new(key: String, value: String) -> MyCookie {
        MyCookie {
            key: key,
            value: value
        }
    }
}



pub fn parse_cookies(value:&str) -> Vec<MyCookie> {
    let mut result: Vec<MyCookie> = Vec::new();
    // get the length
    let len = value.len() - 1;
    println!("{}", len);
    let mut key_cookie = String::from("");
    let mut value_cookie = String::from("");
    let mut found_equal = false;
    // iterate through all the chars
    for (index, c) in value.char_indices() {
        //println!("{} {} {} {}", String::from("index"), index, String::from("char"), c);
        // chekc if it is the space
        if c == ' ' {
            continue;
        }
        // check if it is the end of a key value pair
        if c == ';' {
            let key_clone = key_cookie.clone();
            //let str_key: &str = key_clone.as_str();
            let value_clone = value_cookie.clone();            
            let cookie = MyCookie::new(key_clone, value_clone);
            result.push(cookie);
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
            let cookie = MyCookie::new(key_clone, value_clone);
            result.push(cookie);
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