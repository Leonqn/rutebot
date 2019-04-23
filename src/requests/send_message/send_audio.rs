use std::io::Cursor;
use std::ops::Not;

use hyper::Body;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::Serialize;

use crate::error::Error;
use crate::requests::{add_fields_to_form, add_form_body, add_json_body, ChatId, Request};
use crate::requests::send_message::*;
use crate::responses::Message;

/// Use this struct to send audio files, if you want Telegram clients to display them in the music player.
/// Your audio must be in the .mp3 format. On success, the sent `Message` is returned.
/// Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future
#[derive(Serialize, Debug, Clone)]
pub struct SendAudio<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'i> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Audio file to send.
    #[serde(skip_serializing_if = "FileKind::is_input_file")]
    pub audio: FileKind<'b>,

    /// Audio caption, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'c str>,

    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<&'g str>,

    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'i str>,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using
    /// FileKind::InputFile
    #[serde(skip_serializing_if = "FileKind::is_input_file_or_none")]
    pub thumb: Option<FileKind<'f>>,

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
    pub reply_markup: Option<ReplyMarkup<'d, 'e, 'f>>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'i> Request for SendAudio<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'i> {
    type ResponseType = Message;

    fn method(&self) -> &'static str {
        "sendAudio"
    }

    fn set_http_request_body(self, request_builder: hyper::http::request::Builder) -> Result<hyper::http::request::Request<Body>, Error> {
        if self.audio.is_input_file() || FileKind::is_option_input_file(&self.thumb) {
            let mut form = Form::default();
            add_fields_to_form(&mut form, &self)?;
            if let FileKind::InputFile { name, content } = self.audio {
                form.add_reader_file("audio", Cursor::new(content), name);
            }

            if let Some(FileKind::InputFile { name, content }) = self.thumb {
                form.add_reader_file("thumb", Cursor::new(content), name);
            }
            add_form_body(request_builder, form)
        } else {
            add_json_body(request_builder, &self)
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'i> SendAudio<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'i> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, audio: FileKind<'b>) -> Self {
        Self {
            chat_id: chat_id.into(),
            audio,
            thumb: None,
            caption: None,
            duration: None,
            performer: None,
            disable_notification: false,
            parse_mode: None,
            reply_to_message_id: None,
            reply_markup: None,
            title: None,
        }
    }

    pub fn new_reply(chat_id: impl Into<ChatId<'a>>, audio: FileKind<'b>, reply_to_message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            audio,
            thumb: None,
            caption: None,
            duration: None,
            performer: None,
            disable_notification: false,
            parse_mode: None,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
            title: None,
        }
    }
}