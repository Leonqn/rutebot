use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to get the number of members in a chat. Returns `Int` on success.
#[derive(Serialize, Debug, Clone)]
pub struct GetChatMembersCount<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for GetChatMembersCount<'a> {
    type ResponseType = i64;

    fn method(&self) -> &'static str {
        "getChatMembersCount"
    }
}

impl<'a> GetChatMembersCount<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}
