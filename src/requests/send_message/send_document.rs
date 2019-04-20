use std::io::Cursor;
use std::ops::Not;

use hyper::Body;
use hyper_multipart_rfc7578::client::multipart;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::Serialize;

use crate::requests::{add_fields_to_form, add_json_body, ChatId, Request};
use crate::requests::send_message::*;
use crate::responses::Message;
use crate::error::Error;
use std::error::Error as StdError;

/// Use this method to send general files. On success, the sent `Message` is returned.
/// Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future
#[derive(Serialize, Debug, Clone)]
pub struct SendDocument<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// File to send.
    #[serde(skip_serializing_if = "FileKind::is_input_file")]
    pub document: FileKind<'b>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using
    /// FileKind::InputFile
    #[serde(skip_serializing_if = "FileKind::is_input_file_or_none")]
    pub thumb: Option<FileKind<'f>>,

    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'g str>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in the media caption..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'c, 'd, 'e>>,
}


impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> Request for SendDocument<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendDocument"
    }

    fn set_http_request_body(self, mut request_builder: hyper::http::request::Builder) -> Result<hyper::http::request::Request<Body>, Error> {
        if self.document.is_input_file() || FileKind::is_option_input_file(&self.thumb) {
            let mut form = Form::default();
            add_fields_to_form(&mut form, &self)?;
            if let FileKind::InputFile { name, content } = self.document {
                form.add_reader_file("document", Cursor::new(content), name);
            }

            if let Some(FileKind::InputFile { name, content }) = self.thumb {
                form.add_reader_file("document", Cursor::new(content), name);
            }
            form.set_body_convert::<hyper::Body, multipart::Body>(&mut request_builder)
                .map_err(|x| Error::RequestBuild(x.description().to_string()))
        } else {
            add_json_body(request_builder, &self)
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> SendDocument<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, document: FileKind<'b>) -> Self {
        Self {
            chat_id: chat_id.into(),
            document,
            thumb: None,
            caption: None,
            disable_notification: false,
            parse_mode: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn new_reply(chat_id: impl Into<ChatId<'a>>, document: FileKind<'b>, reply_to_message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            document,
            thumb: None,
            caption: None,
            disable_notification: false,
            parse_mode: None,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        }
    }
}