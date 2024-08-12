use std::num::ParseIntError;
use std::{error::Error, fmt};

use crate::schemas::common::HttpErrorMessage;

#[derive(Debug)]
pub enum OvhManagerError {
    EndpointNotFound,
    ReqwestError(reqwest::Error),
    ParseUrlError,
    ParseIntError(ParseIntError),
    ServerError(HttpErrorMessage),
    SerdeJsonError(serde_json::Error),
}

impl fmt::Display for OvhManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OvhManagerError::EndpointNotFound => write!(f, "Endpoint not found"),
            OvhManagerError::ReqwestError(err) => write!(f, "{err}"),
            OvhManagerError::ParseUrlError => write!(f, "Fail to parse URL"),
            OvhManagerError::ParseIntError(err) => write!(f, "{err}"),
            OvhManagerError::ServerError(err) => write!(f, "{err}"),
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

impl From<serde_json::Error> for OvhManagerError {
    fn from(err: serde_json::Error) -> OvhManagerError {
        OvhManagerError::SerdeJsonError(err)
    }
}

impl From<ParseIntError> for OvhManagerError {
    fn from(err: ParseIntError) -> OvhManagerError {
        OvhManagerError::ParseIntError(err)
    }
}

impl From<HttpErrorMessage> for OvhManagerError {
    fn from(err: HttpErrorMessage) -> OvhManagerError {
        OvhManagerError::ServerError(err)
    }
}
