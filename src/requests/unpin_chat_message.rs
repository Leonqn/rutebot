use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to unpin a message in a group, a supergroup, or a channel.
/// The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’
/// admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct UnpinChatMessage<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for UnpinChatMessage<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "unpinChatMessage"
    }
}

impl<'a> UnpinChatMessage<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}
