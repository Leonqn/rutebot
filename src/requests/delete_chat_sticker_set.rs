use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to delete a group sticker set from a supergroup.
/// The bot must be an administrator in the chat for this to work and must have the appropriate
/// admin rights. Use the field can_set_sticker_set optionally returned in
/// getChat requests to check if the bot can use this method. Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct DeleteChatStickerSet<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for DeleteChatStickerSet<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "deleteChatStickerSet"
    }
}

impl<'a> DeleteChatStickerSet<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self { chat_id: chat_id.into() }
    }
}
