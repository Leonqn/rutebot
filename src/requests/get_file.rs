use serde::Serialize;

use crate::requests::Request;
use crate::responses::File;

/// Use this method to get basic info about a file and prepare it for downloading. For the moment,
/// bots can download files of up to 20MB in size. On success, a `File` object is returned.
/// The file can then be downloaded via the `download_file` method
#[derive(Serialize, Debug, Clone)]
pub struct GetFileRequest<'a> {
    /// File identifier to get info about
    pub file_id: &'a str
}

impl<'a> GetFileRequest<'a> {
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