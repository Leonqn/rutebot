use std::ops::Not;

use serde::Serialize;

use crate::requests::Request;

/// Use this struct to send answers to callback queries sent from inline keyboards.
/// The answer will be displayed to the user as a notification at the top of the chat screen or as an alert.
/// On success, `True` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct AnswerCallbackQuery<'a, 'b, 'c> {
    /// Unique identifier for the query to be answered
    pub callback_query_id: &'a str,

    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'b str>,

    /// If true, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Not::not")]
    pub show_alert: bool,

    /// URL that will be opened by the user's client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'c str>,

    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side.
    /// Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
}


impl<'a, 'b, 'c> Request for AnswerCallbackQuery<'a, 'b, 'c> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "answerCallbackQuery"
    }
}

impl<'a, 'b, 'c> AnswerCallbackQuery<'a, 'b, 'c> {
    pub fn new(query_id: &'a str, notification_text: &'b str) -> Self {
        Self {
            callback_query_id: query_id,
            text: Some(notification_text),
            show_alert: false,
            url: None,
            cache_time: None,
        }
    }
}