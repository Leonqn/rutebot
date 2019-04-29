use serde::Serialize;

use crate::requests::{ChatId, MessageOrInlineMessageId, ReplyMarkup, Request};
use crate::responses::EditedMessage;

/// Use this struct to edit only the reply markup of messages.
/// On success, if edited message is sent by the bot, the edited `Message` is returned, otherwise `True` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct EditMessageReplyMarkup<'a, 'c, 'd, 'e> {
    /// Identifier of message in chat or identifier of inline message
    #[serde(flatten)]
    pub message_or_inline_message_id: MessageOrInlineMessageId<'a>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'c, 'd, 'e>>,
}

impl<'a, 'c, 'd, 'e> Request for EditMessageReplyMarkup<'a, 'c, 'd, 'e> {
    type ResponseType = EditedMessage;

    fn method(&self) -> &'static str {
        "editMessageReplyMarkup"
    }
}

impl<'a, 'b, 'c, 'd, 'e> EditMessageReplyMarkup<'a, 'c, 'd, 'e> {
    pub fn new_inline_message(
        inline_message_id: &'a str,
        reply_markup: ReplyMarkup<'c, 'd, 'e>,
    ) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Inline { inline_message_id },
            reply_markup: Some(reply_markup),
        }
    }

    pub fn new_message(
        chat_id: impl Into<ChatId<'a>>,
        message_id: i64,
        reply_markup: ReplyMarkup<'c, 'd, 'e>,
    ) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Chat {
                chat_id: chat_id.into(),
                message_id,
            },
            reply_markup: Some(reply_markup),
        }
    }
}
