use std::error;
use std::fmt;
use std::time::Duration;

use hyper;
use serde_json;
use tokio;

use crate::responses::ResponseParameters;

/// Contains all possible errors
#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    TokioTimer(tokio::timer::Error),
    TimedOut(Duration),
    Serde(serde_json::Error),
    Api { error_code: i32, description: String, parameters: Option<ResponseParameters> },
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::Hyper(hyper) =>
                Some(hyper),
            Error::Serde(serde) =>
                Some(serde),
            Error::TokioTimer(timer) =>
                Some(timer),
            _ =>
                None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Hyper(hyper) =>
                write!(f, "Hyper error has occurred: {}", hyper),
            Error::TokioTimer(tokio) =>
                write!(f, "Tokio timer error has occurred: {}", tokio),
            Error::TimedOut(timeout) =>
                write!(f, "Request timed out. Provided budget: {} seconds", timeout.as_secs()),
            Error::Serde(serde) =>
                write!(f, "Serde error has occurred: {}", serde),
            Error::Api { error_code, description, parameters } =>
                write!(f, "Error response from telegram bot api: error_code: {:?}, description: {:?}, parameters: {:?}", error_code, description, parameters),
        }
    }
}