use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels,
/// the user will not be able to return to the group on their own using invite links, etc., unless
/// [unbanned](https://core.telegram.org/bots/api#unbanchatmember) first.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns `True` on success.
///
/// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is
/// off in the target group. Otherwise members may only be removed by the group's creator or by the member that added them.
#[derive(Serialize, Debug, Clone)]
pub struct KickChatMember<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or
    /// less than 30 seconds from the current time they are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

impl<'a> Request for KickChatMember<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "kickChatMember"
    }
}

impl<'a> KickChatMember<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
        }
    }
}