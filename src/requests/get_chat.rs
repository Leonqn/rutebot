use serde::Serialize;

use crate::{
    requests::{ChatId, Request},
    responses::Chat,
};

/// Use this struct to get up to date information about the chat
/// (current name of the user for one-on-one
/// conversations, current username of a user, group or channel, etc.). Returns a `Chat` object on success.
#[derive(Serialize, Debug, Clone)]
pub struct GetChat<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for GetChat<'a> {
    type ResponseType = Chat;

    fn method(&self) -> &'static str {
        "getChat"
    }
}

impl<'a> GetChat<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}
