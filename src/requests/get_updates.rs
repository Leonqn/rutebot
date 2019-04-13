use serde::Serialize;

use crate::requests::Request;
use crate::responses::Update;

/// Request to send [getUpdates](https://core.telegram.org/bots/api#getUpdates) request
#[derive(Serialize, Debug, Clone)]
pub struct GetUpdatesRequest<'a> {
    /// Identifier of the first update to be returned. Must be greater by one than the highest
    /// among the identifiers of previously received updates.
    /// By default, updates starting with the earliest unconfirmed update are returned.
    /// An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id.
    /// The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue.
    /// All previous updates will forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Limits the number of updates to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling.
    /// Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// List the types of updates you want your bot to receive.
    /// For example, specify ```&[AllowedUpdate::Message, AllowedUpdate::EditedChannelPost, AllowedUpdate::CallbackQuery]```
    /// to only receive updates of these types.
    /// See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default).
    /// If not specified, the previous setting will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [AllowedUpdate]>,
}

/// Enumeration of possible update types from telegram bot api
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
    /// Create default request.
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
}