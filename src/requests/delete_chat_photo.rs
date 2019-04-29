use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to delete a chat photo. Photos can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights.
/// Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct DeleteChatPhoto<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for DeleteChatPhoto<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "deleteChatPhoto"
    }
}

impl<'a> DeleteChatPhoto<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}
