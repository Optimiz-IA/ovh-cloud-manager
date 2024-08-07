use std::env;
use std::error::Error;

use ovh_cloud_manager::OvhCloudManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let application_key = env::var("OVH_APPLICATION_KEY")?;
    let application_secret = env::var("OVH_APPLICATION_SECRET")?;
    let consumer_key = env::var("OVH_CONSUMER_KEY")?;
    let label_selector = env::var("LABEL_SELECTOR")?;
    let project_id = env::var("OVH_PROJECT_ID")?;

    let ovh_manager = OvhCloudManager::new(
        "ovh-eu",
        &application_key,
        &application_secret,
        &consumer_key,
    )
    .unwrap();

    let project_id_list = ovh_manager.list_project_ids().await?;
    println!("{:?}", project_id_list);

    let job_list = ovh_manager
        .list_ai_jobs(
            &project_id,
            Some(&[("labelSelector", &label_selector), ("statusState", "DONE")]),
        )
        .await?;

    println!("First job: {:#?}", job_list[0]);

    // for job in job_list {
    //     ovh_manager.delete_ai_job(project_id, &job.id).await?;
    // }

    Ok(())
}
