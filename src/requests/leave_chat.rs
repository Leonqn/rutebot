use serde::Serialize;

use crate::requests::{ChatId, Request};

/// Use this method for your bot to leave a group, supergroup or channel. Returns `True` on success.
#[derive(Serialize, Debug, Clone)]
pub struct LeaveChat<'a> {
    /// Unique identifier for the target group or username of the target supergroup or channel
    pub chat_id: ChatId<'a>,
}

impl<'a> Request for LeaveChat<'a> {
    type ResponseType = bool;

    fn method(&self) -> &'static str {
        "leaveChat"
    }
}

impl<'a> LeaveChat<'a> {
    pub fn new(chat_id: impl Into<ChatId<'a>>) -> Self {
        Self { chat_id: chat_id.into() }
    }
}
