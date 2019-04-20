use std::ops::Not;

use serde::Serialize;

/// Contains types for sending [sendMessage](https://core.telegram.org/bots/api#sendMessage) request
pub mod send_text;

/// Contains types for sending [sendDocument](https://core.telegram.org/bots/api#senddocument) request
pub mod send_document;

/// File to send
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum FileKind<'a> {
    /// Identifier of file on the telegram servers
    FileId(&'a str),
    /// Http url for the file to be sent. Telegram will download and send the file.
    /// 5 MB max size for photos and 20 MB max for other types of content
    Url(&'a str),
    /// Arbitrary file to be uploaded
    InputFile {
        /// Name of the file
        name: &'a str,
        /// File content
        content: Vec<u8>,
    },
}


impl<'a> FileKind<'a> {
    pub(crate) fn is_input_file(&self) -> bool {
        if let FileKind::InputFile { .. } = &self {
            true
        } else {
            false
        }
    }

    pub(crate) fn is_input_file_or_none(option_self: &Option<Self>) -> bool {
        match option_self {
            Some(x) => x.is_input_file(),
            None => true
        }
    }

    pub(crate) fn is_option_input_file(option_self: &Option<Self>) -> bool {
        match option_self {
            Some(x) => x.is_input_file(),
            None => false
        }
    }
}

/// Additional interface options
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup<'a, 'b, 'c> {
    InlineKeyboard(InlineKeyboard<'a, 'b, 'c>),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup<'a, 'b>),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

/// This object represents an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating)
/// that appears right next to the message it belongs to
#[derive(Serialize, Debug, Clone)]
pub struct InlineKeyboard<'a, 'b, 'c> {
    /// Array of button rows, each represented by an Array of `InlineKeyboardButton` objects
    pub inline_keyboard: &'c [Vec<InlineKeyboardButton<'a, 'b>>]
}

/// This object represents a custom keyboard with reply options.
#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardMarkup<'a, 'b> {
    /// Array of button rows, each represented by an Array of `KeyboardButton` objects
    pub keyboard: &'b [Vec<KeyboardButton<'a>>],

    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard
    #[serde(skip_serializing_if = "Not::not")]
    pub resize_keyboard: bool,

    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically display the usual
    /// letter-keyboard in the chat – the user can press a special button in the input field to
    /// see the custom keyboard again. Defaults to false
    #[serde(skip_serializing_if = "Not::not")]
    pub one_time_keyboard: bool,
    /// Use this parameter if you want to show the keyboard to specific users only.
    /// Targets: 1) users that are @mentioned in the text of the Message object; 2)
    /// if the bot's message is a reply (has `reply_to_message_id`), sender of the original message.

    /// Example: A user requests to change the bot‘s language, bot replies to the request with a
    /// keyboard to select the new language. Other users in the group don’t see the keyboard
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard
/// and display the default letter-keyboard. By default, custom keyboards are displayed until a
/// new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden
/// immediately after the user presses a button (see `ReplyKeyboardMarkup`)
#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard
    /// (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible,
    /// use `one_time_keyboard` in `ReplyKeyboardMarkup`)
    #[serde(skip_serializing_if = "Not::not")]
    pub remove_keyboard: bool,

    /// Use this parameter if you want to remove the keyboard for specific users only.
    /// Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's
    /// message is a reply (has `reply_to_message_id`), sender of the original message.
    ///
    /// Example: A user votes in a poll, bot returns confirmation message in reply
    /// to the vote and removes the keyboard for that user, while still showing the keyboard
    /// with poll options to users who haven't voted yet
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface
/// to the user (act as if the user has selected the bot‘s message and tapped ’Reply').
/// This can be extremely useful if you want to create user-friendly step-by-step interfaces
/// without having to sacrifice [privacy mode](https://core.telegram.org/bots#privacy-mode).
#[derive(Serialize, Debug, Clone)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot‘s message and tapped ’Reply'
    #[serde(skip_serializing_if = "Not::not")]
    pub force_reply: bool,
    /// Use this parameter if you want to force reply from specific users only.
    /// Targets: 1) users that are @mentioned in the text of the Message object; 2)
    /// if the bot's message is a reply (has `reply_to_message_id`), sender of the original message
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InlineKeyboardButton<'a, 'b> {
    Url {
        /// Label text on the button
        text: &'a str,
        /// HTTP or tg:// url to be opened when button is pressed
        url: &'b str,
    },
    CallbackData {
        /// Label text on the button
        text: &'a str,
        /// Data to be sent in a [callback query](https://core.telegram.org/bots/api#callbackquery)
        /// to the bot when button is pressed, 1-64 bytes
        callback_data: &'b str,
    },
}

/// This object represents one button of the reply keyboard
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeyboardButton<'a> {
    /// Text of the button. It will be sent as a message when the button is pressed
    Text(&'a str)
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ParseMode {
    Html,
    Markdown,
}
