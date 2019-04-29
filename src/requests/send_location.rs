use std::ops::Not;

use serde::Serialize;

use crate::requests::{ChatId, ReplyMarkup, Request};
use crate::responses::Message;

/// Use this struct to send point on the map. On success, the sent `Message` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct SendLocation<'a, 'd, 'e, 'f> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Latitude of the location
    pub latitude: f64,

    /// Longitude of the location
    pub longitude: f64,

    /// Period in seconds for which the location will be updated
    /// (see [Live Locations](https://telegram.org/blog/live-locations), should be between 60 and 86400.
    pub live_period: Option<i64>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'d, 'e, 'f>>,
}

impl<'a, 'd, 'e, 'f> Request for SendLocation<'a, 'd, 'e, 'f> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendLocation"
    }
}

impl<'a, 'd, 'e, 'f> SendLocation<'a, 'd, 'e, 'f> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, latitude: f64, longitude: f64) -> Self {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            live_period: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn new_reply(
        chat_id: impl Into<ChatId<'a>>,
        latitude: f64,
        longitude: f64,
        reply_to_message_id: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            live_period: None,
            disable_notification: false,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        }
    }
}
