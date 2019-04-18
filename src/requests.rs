use serde::Serialize;

/// Contains types for sending [getUpdates](https://core.telegram.org/bots/api#getupdates) request
pub mod get_updates;

/// Contains types for sending [getFile](https://core.telegram.org/bots/api#getfile) request
pub mod get_file;

/// Contains types for sending [getMe](https://core.telegram.org/bots/api#getMe) request
pub mod get_me;

/// Contains types for sending messages
pub mod send_message;

/// Contains types for sending [answerCallbackQuery](https://core.telegram.org/bots/api#answercallbackquery) request
pub mod answer_callback_query;

/// Contains types for sending [sendChatAction](https://core.telegram.org/bots/api#sendchataction) request
pub mod send_chat_action;

/// Basic request type.
pub trait Request: Serialize {
    type ResponseType;

    fn method(&self) -> &'static str;
}

///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId<'a> {
    Id(i64),
    ChannelUsername(&'a str),
}

impl<'a> From<i64> for ChatId<'a> {
    fn from(x: i64) -> Self {
        ChatId::Id(x)
    }
}

impl<'a> From<&'a str> for ChatId<'a> {
    fn from(x: &'a str) -> Self {
        ChatId::ChannelUsername(x)
    }
}