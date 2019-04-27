use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to generate a new invite link for a chat; any previously generated link is revoked.
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the new invite link as
/// `String` on success
#[derive(Serialize, Debug, Clone)]
pub struct ExportChatInviteLink<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for ExportChatInviteLink<'a> {
    type ResponseType = String;

    fn method(&self) -> &'static str {
        "exportChatInviteLink"
    }
}

impl<'a> ExportChatInviteLink<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}