use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::requests::Request;
use crate::responses::Update;

#[derive(Serialize, Debug, Clone)]
pub struct GetUpdatesRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [AllowedUpdate]>,
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
}

impl<'a> Request<Vec<Update>> for GetUpdatesRequest<'a> {
    fn method(&self) -> &'static str {
        "getUpdates"
    }
}

impl<'a> GetUpdatesRequest<'a> {
    pub fn new() -> GetUpdatesRequest<'static> {
        GetUpdatesRequest {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
}