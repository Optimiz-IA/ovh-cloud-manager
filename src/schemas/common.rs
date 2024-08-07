use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct UrlObject {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NameObject {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpErrorMessage {
    pub class: String,
    pub message: String,
}

impl fmt::Display for HttpErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for HttpErrorMessage {}
