use serde::Serialize;

use crate::requests::{ChatId, Request};
use crate::responses::ChatMember;

/// Use this struct to get a list of administrators in a chat. On success,
/// returns an Array of `ChatMember` objects that contains information about all chat
/// administrators except other bots. If the chat is a group or a supergroup and no
/// administrators were appointed, only the creator will be returned.
#[derive(Serialize, Debug, Clone)]
pub struct GetChatAdministrators<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for GetChatAdministrators<'a> {
    type ResponseType = Vec<ChatMember>;

    fn method(&self) -> &'static str {
        "getChatAdministrators"
    }
}

impl<'a> GetChatAdministrators<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}
