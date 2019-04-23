use hyper::Body;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::Serialize;
use serde_json::Value;

use crate::error::Error;
use std::error::Error as StdError;

/// Contains types for sending [getUpdates](https://core.telegram.org/bots/api#getupdates) request
pub mod get_updates;

/// Contains types for sending [getFile](https://core.telegram.org/bots/api#getfile) request
pub mod get_file;

/// Contains types for sending [getMe](https://core.telegram.org/bots/api#getMe) request
pub mod get_me;

/// Contains types for sending messages
pub mod send_message;

/// Contains types for sending [answerCallbackQuery](https://core.telegram.org/bots/api#answercallbackquery) request
pub mod answer_callback_query;

/// Contains types for sending [sendChatAction](https://core.telegram.org/bots/api#sendchataction) request
pub mod send_chat_action;

/// Contains types for sending [forwardMessage](https://core.telegram.org/bots/api#forwardmessage) request
pub mod forward_message;

/// Basic request type.
pub trait Request: Serialize + Sized {
    type ResponseType;

    fn method(&self) -> &'static str;

    fn set_http_request_body(self, request_builder: hyper::http::request::Builder) -> Result<hyper::http::request::Request<Body>, Error> {
        add_json_body(request_builder, &self)
    }
}

pub(crate) fn add_json_body<S: Serialize + Sized>(mut request_builder: hyper::http::request::Builder, serializable: &S) -> Result<hyper::http::request::Request<Body>, Error> {
    let json_bytes = serde_json::to_vec(serializable).map_err(Error::Serde)?;
    request_builder
        .header("content-type", "application/json")
        .body(Body::from(json_bytes))
        .map_err(|x| Error::RequestBuild(x.description().to_string()))
}

pub(crate) fn add_fields_to_form<S: Serialize + Sized>(form: &mut Form<'static>, serializable: &S) -> Result<(), Error> {
    let json = serde_json::to_value(serializable).map_err(Error::Serde)?;
    if let Value::Object(map) = json {
        for (k, v) in map {
            match v {
                Value::String(s) => form.add_text(k, s),
                other => form.add_text(k, other.to_string())
            }
        }
    }
    Ok(())
}

///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId<'a> {
    Id(i64),
    ChannelUsername(&'a str),
}

impl<'a> From<i64> for ChatId<'a> {
    fn from(x: i64) -> Self {
        ChatId::Id(x)
    }
}

impl<'a> From<&'a str> for ChatId<'a> {
    fn from(x: &'a str) -> Self {
        ChatId::ChannelUsername(x)
    }
}