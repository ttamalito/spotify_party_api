

pub struct MyCookie<'a> {
    key: &'a str,
    value: &'a str
}

impl<'a> MyCookie<'a> {
    fn new(key: &'a str, value: &'a str) -> MyCookie<'a> {
        MyCookie {
            key: key,
            value: value
        }
    }
}



pub fn parse_cookies(value:&str) {

    // get the length
    let len = value.len();
    println!("{}", len);
    // iterate through all the chars
    for (size, c) in value.char_indices() {
        println!("{} {} {} {}", String::from("index"), size, String::from("char"), c);
        
    }


}