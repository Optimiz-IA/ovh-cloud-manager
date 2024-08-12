use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::errors::OvhManagerError;
use crate::schemas::common::HttpErrorMessage;

/// The function `response_to_result` in Rust parses a response into a result based on the response
/// status.
///
/// Arguments:
///
/// * `response`: The `response` parameter is of type `Response`, which is typically an HTTP response
///   object returned from a web request.
///
/// Returns:
///
/// The function `response_to_result` returns a `Result` where the success variant contains a
/// deserialized value of type `T` if the response status is successful, and the error variant contains
/// an `OvhManagerError` if the response status is not successful.
pub async fn response_to_result<T: DeserializeOwned>(
    response: Response,
) -> Result<T, OvhManagerError> {
    match response.status().is_success() {
        true => Ok(response.json::<T>().await?),
        false => Err(response.json::<HttpErrorMessage>().await?)?,
    }
}

/// The function `empty_response_to_result` converts an empty response into a Result type in Rust.
///
/// Arguments:
///
/// * `response`: The `response` parameter is of type `Response`, which likely represents an HTTP
///   response received from a server.
///
/// Returns:
///
/// The function `empty_response_to_result` returns a `Result<(), OvhManagerError>`.
pub async fn empty_response_to_result(response: Response) -> Result<(), OvhManagerError> {
    match response.status().is_success() {
        true => Ok(()),
        false => Err(response.json::<HttpErrorMessage>().await?)?,
    }
}
