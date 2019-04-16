use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this struct when you need to tell the user that something is happening on the bot's side.
/// The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status).
/// Returns True on success.
#[derive(Serialize, Debug, Clone)]
pub struct SendChatAction<'a> {
    /// Unique identifier for the target chat
    pub chat_id: ChatId<'a>,

    /// Type of action to broadcast
    pub action: ChatAction,
}

impl<'a> Request for SendChatAction<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "sendChatAction"
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum ChatAction {
    Typing
}


impl<'a> SendChatAction<'a> {
    pub fn new(chat_id: ChatId<'a>, action: ChatAction) -> Self {
        Self {
            chat_id,
            action,
        }
    }
}