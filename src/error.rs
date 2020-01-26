use std::{error, fmt};

use hyper;
use serde_json;

use crate::responses::ResponseParameters;

/// Contains all possible errors
#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    RequestBuilt(String),
    Serde(serde_json::Error),
    /// Telegram bot api error
    Api {
        /// Error code returned by api
        error_code: i32,

        /// Human-readable description of the error
        description: String,

        /// Parameters which can help to automatically handle the error
        parameters: Option<ResponseParameters>,
    },
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::Hyper(hyper) => Some(hyper),
            Error::Serde(serde) => Some(serde),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Hyper(hyper) => write!(f, "Hyper error has occurred: {}", hyper),
            Error::Serde(serde) => write!(f, "Serde error has occurred: {}", serde),
            Error::Api {
                error_code,
                description,
                parameters,
            } => write!(
                f,
                "Error response from telegram bot api: error_code: {:?}, description: {:?}, parameters: {:?}",
                error_code, description, parameters
            ),
            Error::RequestBuilt(x) => write!(f, "Request building was unsuccessful: {}", x),
        }
    }
}
