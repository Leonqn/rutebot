use std::ops::Not;

use serde::Serialize;

use crate::requests::Request;
use crate::responses::{Message, Update};

/// Contains types for sending [sendMessage](https://core.telegram.org/bots/api#sendmessage) request
pub mod send_text_message;

///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId<'a> {
    Id(i64),
    Username(&'a str),
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum FileKind<'a> {
    FileId(&'a str),
    Url(&'a str),
//    InputFile(Vec<u8>),
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ParseMode {
    Html,
    Markdown,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup<'a, 'b, 'c, 'd, 'e> {
    InlineKeyboard(InlineKeyboard<'a, 'b, 'c, 'd, 'e>),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup<'a>),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}


#[derive(Serialize, Debug, Clone)]
pub struct InlineKeyboard<'a, 'b, 'c, 'd, 'e> {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton<'a, 'b, 'c, 'd, 'e>>>
}

#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardMarkup<'a> {
    pub keyboard: Vec<Vec<KeyboardButton<'a>>>,
    #[serde(skip_serializing_if = "Not::not")]
    pub resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardRemove {
    #[serde(skip_serializing_if = "Not::not")]
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ForceReply {
    #[serde(skip_serializing_if = "Not::not")]
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct InlineKeyboardButton<'a, 'b, 'c, 'd, 'e> {
    pub text: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'b str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<&'c str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<&'d str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<&'e str>,
    //    pub callback_game: Option<CallbackGame>
    #[serde(skip_serializing_if = "Not::not")]
    pub pay: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct KeyboardButton<'a> {
    pub text: &'a str,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_location: bool,
}
