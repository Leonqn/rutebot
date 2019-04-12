use serde::Serialize;

use crate::requests::Request;
use crate::responses::File;


#[derive(Serialize, Debug, Clone)]
pub struct GetFileRequest<'a> {
    /// File identifier to get info about
    pub file_id: &'a str
}

impl<'a> GetFileRequest<'a> {
    pub fn new<>(file_id: &'a str) -> Self {
        Self {
            file_id
        }
    }
}

impl<'a> Request<File> for GetFileRequest<'a> {
    fn method(&self) -> &'static str {
        "getFile"
    }
}