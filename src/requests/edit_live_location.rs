use serde::Serialize;

use crate::{
    requests::{ChatId, MessageOrInlineMessageId, ReplyMarkup, Request},
    responses::EditedMessage,
};

/// Use this struct to edit live location messages.
/// A location can be edited until its live_period expires or editing is explicitly disabled by a
/// call to `StopMessageLiveLocation`. On success, if the edited message was sent by the bot,
/// the edited `EditLiveLocationResponse::Message` is returned, otherwise `EditLiveLocationResponse::True` is returned
#[derive(Serialize, Debug, Clone)]
pub struct EditLiveLocation<'a> {
    /// Identifier of message in chat or identifier of inline message
    #[serde(flatten)]
    pub message_or_inline_message_id: MessageOrInlineMessageId<'a>,

    /// Latitude of the location
    pub latitude: f64,

    /// Longitude of the location
    pub longitude: f64,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

impl<'a> Request for EditLiveLocation<'a> {
    type ResponseType = EditedMessage;

    fn method(&self) -> &'static str {
        "editMessageLiveLocation"
    }
}

impl<'a> EditLiveLocation<'a> {
    pub fn new_inline_message(inline_message_id: &'a str, latitude: f64, longitude: f64) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Inline { inline_message_id },
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    pub fn new_message(
        chat_id: impl Into<ChatId<'a>>,
        message_id: i64,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Chat {
                chat_id: chat_id.into(),
                message_id,
            },
            latitude,
            longitude,
            reply_markup: None,
        }
    }
}
