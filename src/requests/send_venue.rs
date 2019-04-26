use std::ops::Not;

use serde::Serialize;

use crate::requests::{ChatId, ReplyMarkup, Request};
use crate::responses::Message;

/// Use this struct to send information about a venue. On success, the sent `Message` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct SendVenue<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Latitude of the location
    pub latitude: f64,

    /// Longitude of the location
    pub longitude: f64,

    /// Name of the venue
    pub title: &'b str,

    /// Address of the venue
    pub address: &'c str,

    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<&'d str>,

    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<&'e str>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'f, 'g, 'h>>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> Request for SendVenue<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendVenue"
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> SendVenue<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, latitude: f64, longitude: f64, title: &'b str, address: &'c str) -> Self {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn new_reply(chat_id: impl Into<ChatId<'a>>, latitude: f64, longitude: f64, title: &'b str, address: &'c str, reply_to_message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: false,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        }
    }
}