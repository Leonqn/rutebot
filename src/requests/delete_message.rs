use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to delete a message, including service messages, with the following limitations:
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - Bots can delete outgoing messages in private chats, groups, and supergroups.
/// - Bots can delete incoming messages in private chats.
/// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
/// Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct DeleteMessage<'a> {
    /// Identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Identifier of the message to delete
    pub message_id: i64,
}

impl<'a> Request for DeleteMessage<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "deleteMessage"
    }
}

impl<'a> DeleteMessage<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
        }
    }
}
