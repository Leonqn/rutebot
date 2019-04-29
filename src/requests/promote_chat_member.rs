use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to promote or demote a user in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights.
/// Pass False for all boolean parameters to demote a user. Returns `True` on success
#[derive(Serialize, Debug, Clone)]
pub struct PromoteChatMember<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// Pass True, if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,

    /// Pass True, if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,

    /// Pass True, if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,

    /// Pass True, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,

    /// Pass True, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,

    /// Pass True, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,

    /// Pass True, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    /// Pass True, if the administrator can add new administrators with a subset of his own privileges
    /// or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
}

impl<'a> Request for PromoteChatMember<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "promoteChatMember"
    }
}

impl<'a> PromoteChatMember<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            can_change_info: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_invite_users: None,
            can_restrict_members: None,
            can_pin_messages: None,
            can_promote_members: None,
        }
    }
}
