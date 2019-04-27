use serde::Serialize;

use crate::requests::{ChatId, MessageOrInlineMessageId, ReplyMarkup, Request};
use crate::responses::EditedMessage;

/// Use this struct to edit live location messages.
/// A location can be edited until its live_period expires or editing is explicitly disabled by a
/// call to `StopMessageLiveLocation`. On success, if the edited message was sent by the bot,
/// the edited `EditLiveLocationResponse::Message` is returned, otherwise `EditLiveLocationResponse::True` is returned
#[derive(Serialize, Debug, Clone)]
pub struct StopLiveLocation<'a, 'd, 'e, 'f> {
    /// Identifier where to stop live location
    #[serde(flatten)]
    pub edit_location_in: MessageOrInlineMessageId<'a>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'d, 'e, 'f>>,
}

impl<'a, 'd, 'e, 'f> Request for StopLiveLocation<'a, 'd, 'e, 'f> {
    type ResponseType = EditedMessage;

    fn method(&self) -> &'static str {
        "stopMessageLiveLocation"
    }
}

impl<'a, 'd, 'e, 'f> StopLiveLocation<'a, 'd, 'e, 'f> {
    pub fn new_inline(inline_message_id: &'a str) -> Self {
        Self {
            edit_location_in: MessageOrInlineMessageId::Inline { inline_message_id },
            reply_markup: None,
        }
    }

    pub fn new_chat(chat_id: impl Into<ChatId<'a>>, message_id: i64) -> Self {
        Self {
            edit_location_in: MessageOrInlineMessageId::Chat { chat_id: chat_id.into(), message_id },
            reply_markup: None,
        }
    }
}