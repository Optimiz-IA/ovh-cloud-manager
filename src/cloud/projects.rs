use std::error::Error;

use super::OvhCloudManager;
use crate::handlers::response_to_result;

impl OvhCloudManager {
    pub async fn list_project_ids(&self) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let response = self.client.get("/cloud/project", None).await?;

        response_to_result(response).await
    }
}
