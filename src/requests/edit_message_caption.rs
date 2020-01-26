use serde::Serialize;

use crate::{
    requests::{ChatId, MessageOrInlineMessageId, ParseMode, ReplyMarkup, Request},
    responses::EditedMessage,
};

/// Use this struct to edit captions of messages. On success,
/// if edited message is sent by the bot, the edited `Message` is returned, otherwise `True` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct EditMessageCaption<'a> {
    /// Identifier of message in chat or identifier of inline message
    #[serde(flatten)]
    pub message_or_inline_message_id: MessageOrInlineMessageId<'a>,

    /// New caption of the message.
    pub caption: Option<&'a str>,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

impl<'a> Request for EditMessageCaption<'a> {
    type ResponseType = EditedMessage;

    fn method(&self) -> &'static str {
        "editMessageCaption"
    }
}

impl<'a> EditMessageCaption<'a> {
    pub fn new_inline_message(inline_message_id: &'a str, caption: &'a str) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Inline { inline_message_id },
            caption: Some(caption),
            parse_mode: None,
            reply_markup: None,
        }
    }

    pub fn new_message(chat_id: impl Into<ChatId<'a>>, message_id: i64, caption: &'a str) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Chat {
                chat_id: chat_id.into(),
                message_id,
            },
            caption: Some(caption),
            parse_mode: None,
            reply_markup: None,
        }
    }
}
