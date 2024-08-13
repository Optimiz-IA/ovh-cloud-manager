use std::num::ParseIntError;
use std::{error::Error, fmt};

use crate::schemas::common::HttpErrorMessage;

#[derive(Debug)]
pub enum OvhManagerError {
    EndpointNotFound(String),
    ParseUrlError(String),
    ReqwestError(reqwest::Error),
    ServerError(HttpErrorMessage),
    ParseServerTimeError(ParseIntError),
    SerdeJsonError(serde_json::Error),
}

impl fmt::Display for OvhManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OvhManagerError::EndpointNotFound(endpoint) => {
                write!(f, "Endpoint \"{endpoint}\" not found")
            }
            OvhManagerError::ParseUrlError(url) => write!(f, "Failed to parse URL: {url}"),
            OvhManagerError::ReqwestError(err) => write!(f, "{err}"),
            OvhManagerError::ServerError(err) => write!(f, "{err}"),
            OvhManagerError::ParseServerTimeError(err) => write!(f, "{err}"),
            OvhManagerError::SerdeJsonError(err) => write!(f, "{err}"),
        }
    }
}

impl Error for OvhManagerError {}

// Implement the conversion from `reqwest::Error` to `OvhManagerError`.
// This will be automatically called by `?` if a `reqwest::Error`
// needs to be converted into a `OvhManagerError`.
impl From<reqwest::Error> for OvhManagerError {
    fn from(err: reqwest::Error) -> OvhManagerError {
        OvhManagerError::ReqwestError(err)
    }
}

// Implement the conversion from `serde_json::Error` to `OvhManagerError`.
// This will be automatically called by `?` if a `serde_json::Error`
// needs to be converted into a `OvhManagerError`.
impl From<serde_json::Error> for OvhManagerError {
    fn from(err: serde_json::Error) -> OvhManagerError {
        OvhManagerError::SerdeJsonError(err)
    }
}

// Implement the conversion from `HttpErrorMessage` to `OvhManagerError`.
// This will be automatically called by `?` if a `HttpErrorMessage`
// needs to be converted into a `OvhManagerError`.
impl From<HttpErrorMessage> for OvhManagerError {
    fn from(err: HttpErrorMessage) -> OvhManagerError {
        OvhManagerError::ServerError(err)
    }
}
