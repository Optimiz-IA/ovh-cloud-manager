/// Macro to handle HTTP responses generically with a fixed error type.
///
/// This macro checks if the HTTP response status indicates success. If it does, it attempts to deserialize
/// the response body into the specified success type. If the status indicates failure, it attempts to
/// deserialize the response body into a fixed error type `HttpErrorMessage` and returns it wrapped in a
/// `Box<dyn Error + Send + Sync>`.
///
/// # Arguments
///
/// * `$response` - The HTTP response expression to handle.
/// * `$success_type` - The type to deserialize a successful response into.
#[macro_export]
macro_rules! handle_response {
    ($response:expr, $success_type:ty) => {
        match $response.status().is_success() {
            true => Ok($response.json::<$success_type>().await?),
            false => Err($response
                .json::<$crate::schemas::common::HttpErrorMessage>()
                .await?)
            .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>),
        }
    };
}

/// Macro to handle HTTP responses that are expected to have empty bodies on success.
///
/// This macro checks if the HTTP response status indicates success. If it does, it returns `Ok(())`.
/// If the status indicates failure, it attempts to deserialize the response body into a fixed error type
/// `HttpErrorMessage` and returns it wrapped in a `Box<dyn Error + Send + Sync>`.
///
/// # Arguments
///
/// * `$response` - The HTTP response expression to handle.
#[macro_export]
macro_rules! handle_empty_response {
    ($response:expr) => {
        match $response.status().is_success() {
            true => Ok(()),
            false => Err($response
                .json::<$crate::schemas::common::HttpErrorMessage>()
                .await?)
            .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>),
        }
    };
}
