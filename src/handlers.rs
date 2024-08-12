use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::errors::OvhManagerError;
use crate::schemas::common::HttpErrorMessage;

pub async fn response_to_result<T: DeserializeOwned>(
    response: Response,
) -> Result<T, OvhManagerError> {
    match response.status().is_success() {
        true => Ok(response.json::<T>().await?),
        false => Err(response.json::<HttpErrorMessage>().await?)?,
    }
}

pub async fn empty_response_to_result(response: Response) -> Result<(), OvhManagerError> {
    match response.status().is_success() {
        true => Ok(()),
        false => Err(response.json::<HttpErrorMessage>().await?)?,
    }
}
