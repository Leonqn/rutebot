use serde::Serialize;

use crate::requests::Request;
use crate::responses::User;

/// Struct to send [getMe](https://core.telegram.org/bots/api#getMe) request
#[derive(Serialize, Debug, Clone, Copy)]
pub struct GetMe;

impl Request<User> for GetMe {
    fn method(&self) -> &'static str {
        "getMe"
    }
}

impl GetMe {
    /// Create default request
    pub fn new() -> Self {
        GetMe
    }
}