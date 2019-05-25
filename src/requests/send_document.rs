use std::ops::Not;

use hyper::Body;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::Serialize;

use crate::error::Error;
use crate::requests::{
    add_fields_to_form, add_file_to_form, add_form_body, add_json_body, ChatId, FileKind,
    ParseMode, ReplyMarkup, Request,
};
use crate::responses::Message;

/// Use this struct to send general files. On success, the sent `Message` is returned.
/// Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future
#[derive(Serialize, Debug, Clone)]
pub struct SendDocument<'a> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// File to send.
    #[serde(skip_serializing_if = "FileKind::is_input_file")]
    pub document: FileKind<'a>,

    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

impl<'a> Request for SendDocument<'a> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendDocument"
    }

    fn set_http_request_body(
        self,
        request_builder: hyper::http::request::Builder,
    ) -> Result<hyper::http::request::Request<Body>, Error> {
        if self.document.is_input_file() {
            let mut form = Form::default();
            add_fields_to_form(&mut form, &self)?;
            add_file_to_form(&mut form, self.document, Some("document"));
            add_form_body(request_builder, form)
        } else {
            add_json_body(request_builder, &self)
        }
    }
}

impl<'a> SendDocument<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, document: FileKind<'a>) -> Self {
        Self {
            chat_id: chat_id.into(),
            document,
            caption: None,
            disable_notification: false,
            parse_mode: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn new_reply(
        chat_id: impl Into<ChatId<'a>>,
        document: FileKind<'a>,
        reply_to_message_id: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            document,
            caption: None,
            disable_notification: false,
            parse_mode: None,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        }
    }
}
