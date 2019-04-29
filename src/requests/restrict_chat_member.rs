use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to restrict a user in a supergroup. The bot must be an administrator in
/// the supergroup for this to work and must have the appropriate admin rights.
/// Pass True for all boolean parameters to lift restrictions from a user. Returns `True` on success
#[derive(Serialize, Debug, Clone)]
pub struct RestrictChatMember<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than
    /// 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,

    /// Pass True, if the user can send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,

    /// Pass True, if the user can send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,

    /// Pass True, if the user can send animations, games, stickers and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,

    /// Pass True, if the user may add web page previews to their messages, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
}

impl<'a> Request for RestrictChatMember<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "restrictChatMember"
    }
}

impl<'a> RestrictChatMember<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
        }
    }
}
