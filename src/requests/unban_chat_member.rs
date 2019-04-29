use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to unban a previously kicked user in a supergroup or channel.
/// The user will not return to the group or channel automatically, but will be able to join via link,
/// etc. The bot must be an administrator for this to work. Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct UnbanChatMember<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// Unique identifier of the target user
    pub user_id: i64,
}

impl<'a> Request for UnbanChatMember<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "unbanChatMember"
    }
}

impl<'a> UnbanChatMember<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}
