use base64::prelude::*;

use std::{str::FromStr, time::Duration};
use serde::{Deserialize, Serialize};
use awc::{Connector, Client};
use openssl::ssl::{SslConnector, SslMethod};

/// Refreshes a token 
/// * token - The Refresh Token saved in the database
/// * code - The code that was received when the first access token was requested
async fn refresh_token(token: String, code: &str) -> bool {

    true
}