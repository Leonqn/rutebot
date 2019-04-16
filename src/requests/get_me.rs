use serde::Serialize;

use crate::requests::Request;
use crate::responses::User;

/// A simple struct for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a `User` object.
#[derive(Serialize, Debug, Clone, Copy)]
pub struct GetMe;

impl Request for GetMe {
    type ResponseType = User;

    fn method(&self) -> &'static str {
        "getMe"
    }
}

impl GetMe {
    pub fn new() -> Self {
        GetMe
    }
}