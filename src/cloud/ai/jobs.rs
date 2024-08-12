use std::error::Error;

use crate::cloud::OvhCloudManager;
use crate::handlers::{empty_response_to_result, response_to_result};
use crate::schemas::ai::jobs::spec::JobSpec;
use crate::schemas::ai::jobs::AIJob;

impl OvhCloudManager {
    pub async fn list_ai_jobs(
        &self,
        project_id: &str,
        parameters: Option<&[(&str, &str)]>,
    ) -> Result<Vec<AIJob>, Box<dyn Error + Send + Sync>> {
        let url = format!("/cloud/project/{}/ai/job", project_id);
        let response = self.client.get(&url, parameters).await?;

        response_to_result(response).await
    }

    pub async fn get_ai_job(
        &self,
        project_id: &str,
        job_id: &str,
    ) -> Result<AIJob, Box<dyn Error + Send + Sync>> {
        let url = format!("/cloud/project/{}/ai/job/{}", project_id, job_id);
        let response = self.client.get(&url, None).await?;

        response_to_result(response).await
    }

    pub async fn create_ai_job(
        &self,
        project_id: &str,
        data: &JobSpec,
    ) -> Result<AIJob, Box<dyn Error + Send + Sync>> {
        let url = format!("/cloud/project/{}/ai/job", project_id);
        let response = self.client.post(&url, data).await?;

        response_to_result(response).await
    }

    pub async fn delete_ai_job(
        &self,
        project_id: &str,
        job_id: &str,
        force: bool,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = format!("/cloud/project/{}/ai/job/{}", project_id, job_id);

        let response = match force {
            true => self.client.delete(&url, Some(&[("force", "true")])).await?,
            false => self.client.delete(&url, None).await?,
        };

        empty_response_to_result(response).await
    }
}
