use std::error::Error;

use crate::cloud::OvhCloudManager;
use crate::schemas::ai::data::{DatastoreAlias, FormDatastoreAlias};
use crate::{handle_empty_response, handle_response};

impl OvhCloudManager {
    pub async fn list_ai_aliases(
        &self,
        project_id: &str,
        region: &str,
    ) -> Result<Vec<DatastoreAlias>, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "/cloud/project/{}/ai/data/region/{}/alias",
            project_id, region
        );
        let response = self.client.get(&url, None).await?;

        handle_response!(response, Vec<DatastoreAlias>)
    }

    pub async fn get_ai_alias(
        &self,
        project_id: &str,
        region: &str,
        alias: &str,
    ) -> Result<DatastoreAlias, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "/cloud/project/{}/ai/data/region/{}/alias/{}",
            project_id, region, alias
        );
        let response = self.client.get(&url, None).await?;

        handle_response!(response, DatastoreAlias)
    }

    pub async fn create_ai_alias(
        &self,
        project_id: &str,
        region: &str,
        data: &FormDatastoreAlias,
    ) -> Result<DatastoreAlias, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "/cloud/project/{}/ai/data/region/{}/alias",
            project_id, region
        );
        let response = self.client.post(&url, data).await?;

        handle_response!(response, DatastoreAlias)
    }

    pub async fn update_ai_alias(
        &self,
        project_id: &str,
        region: &str,
        alias: &str,
        data: &FormDatastoreAlias,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = format!(
            "/cloud/project/{}/ai/data/region/{}/alias/{}",
            project_id, region, alias
        );
        let response = self.client.post(&url, data).await?;

        handle_empty_response!(response)
    }

    pub async fn delete_ai_alias(
        &self,
        project_id: &str,
        region: &str,
        alias: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = format!(
            "/cloud/project/{}/ai/data/region/{}/alias/{}",
            project_id, region, alias
        );
        let response = self.client.delete(&url, None).await?;

        handle_empty_response!(response)
    }
}
