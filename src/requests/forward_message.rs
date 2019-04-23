use std::ops::Not;

use serde::Serialize;

use crate::requests::{ChatId, Request};
use crate::responses::Message;

/// Use this struct to forward messages of any kind. On success, the sent `Message` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct ForwardMessage<'a, 'b> {
    /// Unique identifier for the target chat
    chat_id: ChatId<'a>,

    /// Unique identifier for the chat where the original message was sent
    from_chat_id: ChatId<'b>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,

    /// Message identifier in the chat specified in from_chat_id
    message_id: i64,
}

impl<'a, 'b> Request for ForwardMessage<'a, 'b> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "forwardMessage"
    }
}

impl<'a, 'b> ForwardMessage<'a, 'b> {
    pub fn new(from: impl Into<ChatId<'b>>, to: impl Into<ChatId<'a>>, message_id: i64) -> Self {
        Self {
            chat_id: to.into(),
            from_chat_id: from.into(),
            disable_notification: false,
            message_id,
        }
    }
}