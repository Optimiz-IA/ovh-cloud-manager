use std::error::Error;

use super::OvhCloudManager;
use crate::handle_response;

impl OvhCloudManager {
    pub async fn list_project_ids(&self) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let response = self.client.get("/cloud/project", None).await?;

        handle_response!(response, Vec<String>)
    }
}
