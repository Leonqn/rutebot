use std::ops::Not;

use serde::Serialize;

use crate::{
    requests::{ChatId, ParseMode, ReplyMarkup, Request},
    responses::Message,
};

/// Use this struct to send text messages. On success, the sent `Message` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct SendMessage<'a> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Text of the message to be sent.
    pub text: &'a str,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

impl<'a> Request for SendMessage<'a> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendMessage"
    }
}

impl<'a> SendMessage<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, text: &'a str) -> Self {
        Self {
            chat_id: chat_id.into(),
            text,
            disable_notification: false,
            parse_mode: None,
            disable_web_page_preview: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn new_reply(
        chat_id: impl Into<ChatId<'a>>,
        text: &'a str,
        reply_to_message_id: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            text,
            disable_notification: false,
            parse_mode: None,
            disable_web_page_preview: false,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        }
    }
}
