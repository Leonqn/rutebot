use serde::Serialize;

use crate::requests::Request;
use crate::responses::User;

/// Represents [getMe](https://core.telegram.org/bots/api#getMe) request
#[derive(Serialize, Debug, Clone, Copy)]
pub struct GetMe;

impl Request for GetMe {
    type ResponseType = User;

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