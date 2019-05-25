use std::ops::Not;

use serde::Serialize;

use crate::requests::{ChatId, MessageOrInlineMessageId, ParseMode, ReplyMarkup, Request};
use crate::responses::EditedMessage;

/// Use this struct to edit text and game messages. On success, if edited message is sent by the bot, the edited
/// `Message `is returned, otherwise `True` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct EditMessageText<'a> {
    /// Identifier of message in chat or identifier of inline message
    #[serde(flatten)]
    pub message_or_inline_message_id: MessageOrInlineMessageId<'a>,

    /// New text of the message.
    pub text: &'a str,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

impl<'a> Request for EditMessageText<'a> {
    type ResponseType = EditedMessage;

    fn method(&self) -> &'static str {
        "editMessageText"
    }
}

impl<'a> EditMessageText<'a> {
    pub fn new_inline_message(inline_message_id: &'a str, text: &'a str) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Inline { inline_message_id },
            text,
            parse_mode: None,
            disable_web_page_preview: false,
            reply_markup: None,
        }
    }

    pub fn new_message(chat_id: impl Into<ChatId<'a>>, message_id: i64, text: &'a str) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Chat {
                chat_id: chat_id.into(),
                message_id,
            },
            text,
            parse_mode: None,
            disable_web_page_preview: false,
            reply_markup: None,
        }
    }
}
