use reqwest::Response;
use serde::de::DeserializeOwned;
use std::error::Error;

use crate::schemas::common::HttpErrorMessage;

/// The function `response_to_result` in Rust parses a response into a result based on the response
/// status.
///
/// Arguments:
///
/// * `response`: The `response` parameter in the `response_to_result` function is of type `Response`,
/// which represents an HTTP response.
///
/// Returns:
///
/// The function `response_to_result` returns a `Result` where the success variant contains a
/// deserialized value of type `T` if the response status is successful, and the error variant contains
/// a deserialized `HttpErrorMessage` if the response status is not successful. The error type is a
/// boxed trait object that implements `Error`, `Send`, and `Sync`.
pub async fn response_to_result<T: DeserializeOwned>(
    response: Response,
) -> Result<T, Box<dyn Error + Send + Sync>> {
    match response.status().is_success() {
        true => Ok(response.json::<T>().await?),
        false => Err(response.json::<HttpErrorMessage>().await?)
            .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>),
    }
}

/// The function `empty_response_to_result` in Rust checks if a response is successful and returns a
/// result accordingly.
///
/// Arguments:
///
/// * `response`: The `response` parameter is of type `Response`, which likely represents an HTTP
/// response received from a server in an asynchronous context.
///
/// Returns:
///
/// The function `empty_response_to_result` returns a `Result` with the success case containing `()`
/// (unit type) if the response status is successful, and an error containing a boxed `HttpErrorMessage`
/// if the response status is not successful. The error type is `Box<dyn Error + Send + Sync>`.
pub async fn empty_response_to_result(
    response: Response,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match response.status().is_success() {
        true => Ok(()),
        false => Err(response.json::<HttpErrorMessage>().await?)
            .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>),
    }
}
