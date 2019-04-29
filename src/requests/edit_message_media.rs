use std::io::Cursor;

use hyper::Body;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::Serialize;

use crate::error::Error;
use crate::requests::{
    add_fields_to_form, add_form_body, add_json_body, ChatId, FileKind, InputMedia,
    MessageOrInlineMessageId, ReplyMarkup, Request,
};
use crate::responses::EditedMessage;

/// Use this struct to edit animation, audio, document, photo, or video messages.
/// If a message is a part of a message album, then it can be edited only to a photo or a video.
/// Otherwise, message type can be changed arbitrarily. When inline message is edited, new file
/// can't be uploaded. Use previously uploaded file via its file_id or specify a URL. On success,
/// if the edited message was sent by the bot, the edited `Message` is returned, otherwise `True` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct EditMessageMedia<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    /// Identifier of message in chat or identifier of inline message
    #[serde(flatten)]
    pub message_or_inline_message_id: MessageOrInlineMessageId<'a>,

    /// New media content of the message.
    pub media: InputMedia<'b, 'c, 'd, 'e>,

    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup<'f, 'g, 'h>>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> Request for EditMessageMedia<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    type ResponseType = EditedMessage;

    fn method(&self) -> &'static str {
        "editMessageMedia"
    }

    fn set_http_request_body(
        self,
        request_builder: hyper::http::request::Builder,
    ) -> Result<hyper::http::request::Request<Body>, Error> {
        if self.media.contains_input_file() {
            let mut form = Form::default();
            add_fields_to_form(&mut form, &self)?;
            if let FileKind::InputFile { name, content } = self.media.get_file() {
                form.add_reader_file(name, Cursor::new(content), name);
            }

            add_form_body(request_builder, form)
        } else {
            add_json_body(request_builder, &self)
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> EditMessageMedia<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    pub fn new_inline_message(
        inline_message_id: &'a str,
        media: InputMedia<'b, 'c, 'd, 'e>,
    ) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Inline { inline_message_id },
            media,
            reply_markup: None,
        }
    }

    pub fn new_message(
        chat_id: impl Into<ChatId<'a>>,
        message_id: i64,
        media: InputMedia<'b, 'c, 'd, 'e>,
    ) -> Self {
        Self {
            message_or_inline_message_id: MessageOrInlineMessageId::Chat {
                chat_id: chat_id.into(),
                message_id,
            },
            media,
            reply_markup: None,
        }
    }
}
