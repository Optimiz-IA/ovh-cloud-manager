use crate::cloud::OvhCloudManager;
use crate::errors::OvhManagerError;
use crate::handlers::response_to_result;
use crate::schemas::ai::capabilities::AIFlavor;
use crate::schemas::ai::AIQuota;

impl OvhCloudManager {
    pub async fn get_ai_quota(&self, project_id: &str) -> Result<AIQuota, OvhManagerError> {
        let url = format!("/cloud/project/{}/ai/capabilities/quota", project_id);
        let response = self.client.get(&url, None).await?;

        response_to_result(response).await
    }

    pub async fn list_ai_flavors(
        &self,
        project_id: &str,
        region: &str,
    ) -> Result<Vec<AIFlavor>, OvhManagerError> {
        let url = format!(
            "/cloud/project/{}/ai/capabilities/region/{}/flavor",
            project_id, region
        );
        let response = self.client.get(&url, None).await?;

        response_to_result(response).await
    }

    pub async fn get_ai_flavor_details(
        &self,
        project_id: &str,
        region: &str,
        flavor_id: &str,
    ) -> Result<AIFlavor, OvhManagerError> {
        let url = format!(
            "/cloud/project/{}/ai/capabilities/region/{}/flavor/{}",
            project_id, region, flavor_id
        );
        let response = self.client.get(&url, None).await?;

        response_to_result(response).await
    }
}
