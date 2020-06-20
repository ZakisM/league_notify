use core::fmt;

use serde::export::Formatter;

#[derive(Debug)]
pub struct ApiError {
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ApiError {
    pub fn new<S: AsRef<str>>(message: S) -> Self {
        Self {
            message: message.as_ref().to_string(),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(req_err: reqwest::Error) -> Self {
        Self {
            message: req_err.to_string(),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(serde_err: serde_json::Error) -> Self {
        Self {
            message: serde_err.to_string(),
        }
    }
}

impl From<std::string::String> for ApiError {
    fn from(s: String) -> Self {
        Self { message: s }
    }
}
