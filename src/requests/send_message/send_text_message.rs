use serde::Serialize;
use std::ops::Not;

use crate::requests::Request;
use crate::responses::{Message, Update};
use crate::requests::send_message::*;

#[derive(Serialize, Debug, Clone)]
pub struct SendTextMessageRequest<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Text of the message to be sent.
    text: &'b str,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in your bot's message.
    pub parse_mode: Option<ParseMode>,

    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'c, 'd, 'e, 'f, 'g>>,
}


impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> Request<Message> for SendTextMessageRequest<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    fn method(&self) -> &'static str {
        "sendMessage"
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> SendTextMessageRequest<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub fn new(chat_id: ChatId<'a>, text: &'b str) -> Self {
        Self {
            chat_id,
            text,
            disable_notification: false,
            parse_mode: None,
            disable_web_page_preview: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}