use serde::Serialize;

use crate::requests::{ChatId, ReplyMarkup, Request};
use crate::responses::Poll;

/// Use this struct to stop a poll which was sent by the bot. On success, the stopped `Poll` with the final results is returned.
#[derive(Serialize, Debug, Clone)]
pub struct StopPoll<'a, 'f, 'g, 'h> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Identifier of the original message with the poll
    pub message_id: i64,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'f, 'g, 'h>>,
}

impl<'a, 'f, 'g, 'h> Request for StopPoll<'a, 'f, 'g, 'h> {
    type ResponseType = Poll;

    fn method(&self) -> &'static str {
        "stopPoll"
    }
}

impl<'a, 'f, 'g, 'h> StopPoll<'a, 'f, 'g, 'h> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }
}