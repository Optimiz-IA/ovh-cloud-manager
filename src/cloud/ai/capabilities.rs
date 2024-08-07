use std::error::Error;

use crate::cloud::OvhCloudManager;
use crate::handle_response;
use crate::schemas::ai::capabilities::AIFlavor;
use crate::schemas::ai::AIQuota;

impl OvhCloudManager {
    pub async fn get_ai_quota(
        &self,
        project_id: &str,
    ) -> Result<AIQuota, Box<dyn Error + Send + Sync>> {
        let url = format!("/cloud/project/{}/ai/capabilities/quota", project_id);
        let response = self.client.get(&url, None).await?;

        handle_response!(response, AIQuota)
    }

    pub async fn list_ai_flavors(
        &self,
        project_id: &str,
        region: &str,
    ) -> Result<Vec<AIFlavor>, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "/cloud/project/{}/ai/capabilities/region/{}/flavor",
            project_id, region
        );
        let response = self.client.get(&url, None).await?;

        handle_response!(response, Vec<AIFlavor>)
    }

    pub async fn get_ai_flavor_details(
        &self,
        project_id: &str,
        region: &str,
        flavor_id: &str,
    ) -> Result<AIFlavor, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "/cloud/project/{}/ai/capabilities/region/{}/flavor/{}",
            project_id, region, flavor_id
        );
        let response = self.client.get(&url, None).await?;

        handle_response!(response, AIFlavor)
    }
}
