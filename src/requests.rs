use serde::Serialize;
use serde::de::DeserializeOwned;

/// Contains types for sending [getUpdates](https://core.telegram.org/bots/api#getupdates) request
pub mod get_updates;
/// Contains types for sending [getFile](https://core.telegram.org/bots/api#getfile) request
pub mod get_file;

/// Basic request type.
pub trait Request<Response: DeserializeOwned>: Serialize {
    fn method(&self) -> &'static str;
}