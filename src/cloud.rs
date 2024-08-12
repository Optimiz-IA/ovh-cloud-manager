pub mod ai;
pub mod projects;

use crate::client::OvhClient;
use crate::errors::OvhManagerError;

pub struct OvhCloudManager {
    client: OvhClient,
}

impl OvhCloudManager {
    pub fn new(
        endpoint: &str,
        application_key: &str,
        application_secret: &str,
        consumer_key: &str,
    ) -> Result<Self, OvhManagerError> {
        Ok(Self {
            client: OvhClient::new(endpoint, application_key, application_secret, consumer_key)?,
        })
    }
}
