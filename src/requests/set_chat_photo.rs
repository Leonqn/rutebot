use std::io::Cursor;

use hyper::Body;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::Serialize;

use crate::error::Error;
use crate::requests::{add_fields_to_form, add_form_body, ChatId, Request};

/// Use this struct to set a new profile photo for the chat. Photos can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
///
/// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group.
#[derive(Serialize, Debug, Clone)]
pub struct SetChatPhoto<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// New chat photo content
    #[serde(skip_serializing)]
    pub photo: Vec<u8>,
}

impl<'a> Request for SetChatPhoto<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "setChatPhoto"
    }

    fn set_http_request_body(self, request_builder: hyper::http::request::Builder) -> Result<hyper::http::request::Request<Body>, Error> {
        let mut form = Form::default();
        add_fields_to_form(&mut form, &self)?;
        form.add_reader_file("photo", Cursor::new(self.photo), "photo");
        add_form_body(request_builder, form)
    }
}

impl<'a> SetChatPhoto<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, photo: Vec<u8>) -> Self {
        Self {
            chat_id: chat_id.into(),
            photo,
        }
    }
}