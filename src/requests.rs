use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod get_updates;
pub mod get_file;


pub trait Request<Response: DeserializeOwned>: Serialize {
    fn method(&self) -> &'static str;
}