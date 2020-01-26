use std::ops::Not;

use serde::Serialize;

use crate::{
    requests::{ChatId, ReplyMarkup, Request},
    responses::Message,
};

/// Use this struct to send a native poll. A native poll can't be sent to a private chat.
/// On success, the sent `Message` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct SendPoll<'a> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Poll question, 1-255 characters
    pub question: &'a str,

    /// List of answer options, 2-10 strings 1-100 characters each
    pub options: &'a [&'a str],

    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

impl<'a> Request for SendPoll<'a> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendPoll"
    }
}

impl<'a> SendPoll<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, question: &'a str, options: &'a [&'a str]) -> Self {
        Self {
            chat_id: chat_id.into(),
            question,
            options,
            last_name: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn new_reply(
        chat_id: impl Into<ChatId<'a>>,
        question: &'a str,
        options: &'a [&'a str],
        reply_to_message_id: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            question,
            options,
            last_name: None,
            disable_notification: false,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        }
    }
}
