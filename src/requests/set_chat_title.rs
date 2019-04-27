use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to change the title of a chat. Titles can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights.
/// Returns `True` on success.
///
/// Note: In regular groups (non-supergroups), this method will only work if the
/// ‘All Members Are Admins’ setting is off in the target group.
#[derive(Serialize, Debug, Clone)]
pub struct SetChatTitle<'a, 'b> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// New chat title, 1-255 characters
    pub title: &'b str,
}

impl<'a, 'b> Request for SetChatTitle<'a, 'b> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "setChatTitle"
    }
}

impl<'a, 'b> SetChatTitle<'a, 'b> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, title: &'b str) -> Self {
        Self {
            chat_id: chat_id.into(),
            title,
        }
    }
}