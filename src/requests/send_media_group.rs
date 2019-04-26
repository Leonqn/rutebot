use std::io::Cursor;
use std::ops::Not;

use hyper::Body;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::Serialize;

use crate::error::Error;
use crate::requests::{add_fields_to_form, add_form_body, add_json_body, ChatId, FileKind, InputMediaPhoto, InputMediaVideo, Request};
use crate::responses::Message;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InputMediaPhotoOrVideo<'a, 'b> {
    #[serde(rename = "video")]
    Video(InputMediaVideo<'a, 'b>),
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto<'a, 'b>),
}

impl<'a, 'b> InputMediaPhotoOrVideo<'a, 'b> {
    fn contains_input_file(&self) -> bool {
        match &self {
            InputMediaPhotoOrVideo::Video(_) => false,
            InputMediaPhotoOrVideo::Photo(x) => x.media.is_input_file()
        }
    }

    fn get_file(self) -> FileKind<'a> {
        match self {
            InputMediaPhotoOrVideo::Photo(x) => x.media,
            InputMediaPhotoOrVideo::Video(x) => x.media
        }
    }
}

/// Use this method to send a group of photos or videos as an album.
/// On success, an array of the sent `Messages` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct SendMediaGroup<'a, 'b, 'c> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Photos and videos to be sent, must include 2–10 items
    pub media: Vec<InputMediaPhotoOrVideo<'b, 'c>>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

}

impl<'a, 'b, 'c> Request for SendMediaGroup<'a, 'b, 'c> {
    type ResponseType = Vec<Message>;

    fn method(&self) -> &'static str {
        "sendMediaGroup"
    }

    fn set_http_request_body(self, request_builder: hyper::http::request::Builder) -> Result<hyper::http::request::Request<Body>, Error> {
        if self.media.iter().any(|x| x.contains_input_file()) {
            let mut form = Form::default();
            add_fields_to_form(&mut form, &self)?;
            for media in self.media.into_iter() {
                if let FileKind::InputFile { name, content } = media.get_file() {
                    form.add_reader_file(name, Cursor::new(content), name);
                }
            }

            add_form_body(request_builder, form)
        } else {
            add_json_body(request_builder, &self)
        }
    }
}

impl<'a, 'b, 'c> SendMediaGroup<'a, 'b, 'c> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, photo_or_video: Vec<InputMediaPhotoOrVideo<'b, 'c>>) -> Self {
        Self {
            chat_id: chat_id.into(),
            media: photo_or_video,
            disable_notification: false,
            reply_to_message_id: None
        }
    }

    pub fn new_reply(chat_id: impl Into<ChatId<'a>>, photo_or_video: Vec<InputMediaPhotoOrVideo<'b, 'c>>, reply_to_message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            media: photo_or_video,
            disable_notification: false,
            reply_to_message_id: Some(reply_to_message_id),
        }
    }
}