use std::ops::Not;

use serde::Serialize;

use crate::requests::{ChatId, ReplyMarkup, Request};
use crate::responses::Message;

/// Use this struct to send phone contacts. On success, the sent `Message` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct SendContact<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Contact's phone number
    pub phone_number: &'b str,

    /// Contact's first name
    pub first_name: &'c str,

    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'d str>,

    /// Additional data about the contact in the form of a
    /// [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<&'e str>,

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

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> Request for SendContact<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendContact"
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> SendContact<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, phone_number: &'b str, first_name: &'c str) -> Self {
        Self {
            chat_id: chat_id.into(),
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn new_reply(
        chat_id: impl Into<ChatId<'a>>,
        phone_number: &'b str,
        first_name: &'c str,
        reply_to_message_id: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            disable_notification: false,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        }
    }
}
