use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct to set a new group sticker set for a supergroup.
/// The bot must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Use the field `can_set_sticker_set optionally` returned in
/// getChat requests to check if the bot can use this method. Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct SetChatStickerSet<'a, 'b> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,

    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: &'b str,
}

impl<'a, 'b> Request for SetChatStickerSet<'a, 'b> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "setChatStickerSet"
    }
}

impl<'a, 'b> SetChatStickerSet<'a, 'b> {
    pub fn new(chat_id: impl Into<ChatId<'a>>, sticker_set_name: &'b str) -> Self {
        Self {
            chat_id: chat_id.into(),
            sticker_set_name,
        }
    }
}