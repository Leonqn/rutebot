use std::ops::Not;

use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to pin a message in a group, a supergroup, or a channel.
/// The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’
/// admin right in the supergroup or ‘can_edit_messages’ admin right in the channel.
/// Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct PinChatMessage<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// Identifier of a message to pin
    pub message_id: i64,

    /// Pass True, if it is not necessary to send a notification to all chat members about
    /// the new pinned message. Notifications are always disabled in channels..
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,
}

impl<'a> Request for PinChatMessage<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "pinChatMessage"
    }
}

impl<'a> PinChatMessage<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            disable_notification: false,
        }
    }
}
