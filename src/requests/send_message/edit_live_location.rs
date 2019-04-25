use serde::Serialize;

use crate::requests::{ChatId, EditLocationIn, ReplyMarkup, Request};
use crate::responses::EditedLiveLocation;

/// Use this struct to edit live location messages.
/// A location can be edited until its live_period expires or editing is explicitly disabled by a
/// call to `StopMessageLiveLocation`. On success, if the edited message was sent by the bot,
/// the edited `EditLiveLocationResponse::Message` is returned, otherwise `EditLiveLocationResponse::True` is returned
#[derive(Serialize, Debug, Clone)]
pub struct EditLiveLocation<'a, 'd, 'e, 'f> {
    /// Identifier where to edit live location
    #[serde(flatten)]
    pub edit_location_in: EditLocationIn<'a>,

    /// Latitude of the location
    pub latitude: f64,

    /// Longitude of the location
    pub longitude: f64,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'d, 'e, 'f>>,
}

impl<'a, 'd, 'e, 'f> Request for EditLiveLocation<'a, 'd, 'e, 'f> {
    type ResponseType = EditedLiveLocation;

    fn method(&self) -> &'static str {
        "editMessageLiveLocation"
    }
}

impl<'a, 'd, 'e, 'f> EditLiveLocation<'a, 'd, 'e, 'f> {
    pub fn new_inline(inline_message_id: &'a str, latitude: f64, longitude: f64) -> Self {
        Self {
            edit_location_in: EditLocationIn::Inline { inline_message_id },
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    pub fn new_chat(chat_id: impl Into<ChatId<'a>>, message_id: i64, latitude: f64, longitude: f64) -> Self {
        Self {
            edit_location_in: EditLocationIn::Chat { chat_id: chat_id.into(), message_id },
            latitude,
            longitude,
            reply_markup: None,
        }
    }
}