use serde::Serialize;

use crate::requests::{ChatId, Request};

#[derive(Serialize, Debug, Clone)]
pub struct SendChatAction<'a> {
    pub chat_id: ChatId<'a>,
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