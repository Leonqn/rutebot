use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to change the description of a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights.
/// Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct SetChatDescription<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// New chat description, 0-255 characters
    pub description: Option<&'a str>,
}

impl<'a> Request for SetChatDescription<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "setChatDescription"
    }
}

impl<'a> SetChatDescription<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
            description: None,
        }
    }
    pub fn new_description(chat_id: impl Into<ChatId<'a>>, description: &'a str) -> Self {
        Self {
            chat_id: chat_id.into(),
            description: Some(description),
        }
    }
}
