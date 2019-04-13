use serde::Serialize;

use crate::requests::Request;
use crate::responses::File;

/// Represents [getFile](https://core.telegram.org/bots/api#getfile) request
#[derive(Serialize, Debug, Clone)]
pub struct GetFileRequest<'a> {
    /// File identifier to get info about
    pub file_id: &'a str
}

impl<'a> GetFileRequest<'a> {
    /// Create request with given file_id
    pub fn new(file_id: &'a str) -> Self {
        Self {
            file_id
        }
    }
}

impl<'a> Request for GetFileRequest<'a> {
    type ResponseType = File;

    /// Returns telegram bot api method name
    fn method(&self) -> &'static str {
        "getFile"
    }
}