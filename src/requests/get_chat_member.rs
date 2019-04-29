use serde::Serialize;

use crate::requests::{ChatId, Request};
use crate::responses::ChatMember;

/// Use this method to get information about a member of a chat. Returns a `ChatMember` object on success.
#[derive(Serialize, Debug, Clone)]
pub struct GetChatMember<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// Unique identifier of the target user
    pub user_id: i64,
}

impl<'a> Request for GetChatMember<'a> {
    type ResponseType = ChatMember;

    fn method(&self) -> &'static str {
        "getChatMember"
    }
}

impl<'a> GetChatMember<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}
