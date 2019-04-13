use serde::de::DeserializeOwned;
use serde::Serialize;

/// Contains types for sending [getUpdates](https://core.telegram.org/bots/api#getupdates) request
pub mod get_updates;

/// Contains types for sending [getFile](https://core.telegram.org/bots/api#getfile) request
pub mod get_file;

/// Contains types for sending [getMe](https://core.telegram.org/bots/api#getMe) request
pub mod get_me;

/// Contains types for sending messages
pub mod send_message;

/// Basic request type.
pub trait Request<Response: DeserializeOwned>: Serialize {
    fn method(&self) -> &'static str;
}