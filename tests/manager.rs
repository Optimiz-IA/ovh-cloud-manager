#[cfg(test)]
mod tests {
    use ovh_cloud_manager::OvhCloudManager;
    use std::env;

    struct TestConfig {
        region: String,
        application_key: String,
        application_secret: String,
        consumer_key: String,
    }

    impl TestConfig {
        fn new() -> Self {
            dotenvy::dotenv().unwrap();

            Self {
                region: env::var("OVH_REGION").unwrap(),
                application_key: env::var("OVH_APPLICATION_KEY").unwrap(),
                application_secret: env::var("OVH_APPLICATION_SECRET").unwrap(),
                consumer_key: env::var("OVH_CONSUMER_KEY").unwrap(),
            }
        }
    }

    fn init_manager() -> OvhCloudManager {
        let creds = TestConfig::new();

        OvhCloudManager::new(
            &creds.region,
            &creds.application_key,
            &creds.application_secret,
            &creds.consumer_key,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn list_projects() {
        let manager = init_manager();
        let res = manager.list_project_ids().await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn list_jobs() {
        let manager = init_manager();
        let project_id = env::var("OVH_PROJECT_ID").unwrap();
        let res = manager.list_ai_jobs(&project_id, None).await;

        assert!(res.is_ok())
    }
}
