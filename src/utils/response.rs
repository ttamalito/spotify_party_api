use actix_web::web::to;
use jwt::token;
use serde::Serialize;

/// Struct to Represent a simple respose from the api
#[derive(Serialize)]
pub struct JsonResponse {
    result: bool,
    redirected: bool,
    url: String
}

/// impl block for static methods
impl JsonResponse {
    pub fn new(result: bool, redirected: bool, url: String) -> Self {
        JsonResponse {
            result,
            redirected, 
            url 
        }
    } // end of new

    /// Creates a simple JSON response, where the result is true
    pub fn simple_response() -> Self {
        JsonResponse::new(true, false, String::from(""))
    }

    /// Create a JSON response to redirect to login
    pub fn redirect_to_login() -> Self {
        JsonResponse::new(false, true, String::from("http://localhost:3000/login"))
    }
}

impl JsonResponse {
    pub fn get_result(&self) -> bool {
        self.result
    }

    pub fn get_redirected(&self) -> bool {
        self.redirected
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

#[derive(Serialize)]
pub struct JsonResponseForSigningUp {
    result: bool,
    redirected: bool,
    url: String,
    token: String,
    id: String
}


impl JsonResponseForSigningUp {
    pub fn new(result: bool, redirected: bool, url: String, token: String, id: String) -> Self {
        JsonResponseForSigningUp {
            result,
            redirected, 
            url,
            token,
            id 
        }
    } // end of new

    pub fn redirect_to_login(token: String, id: String) -> Self {
        JsonResponseForSigningUp::new(true, true, String::from("http://localhost:3000/login"), token, id)
    }
    
}