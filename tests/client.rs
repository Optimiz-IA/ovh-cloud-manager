#[cfg(test)]
mod tests {
    use ovh_cloud_manager::client::OvhClient;
    use ovh_cloud_manager::errors::OvhManagerError;

    #[tokio::test]
    async fn endpoint_not_found() {
        let res = OvhClient::new(
            "ovh-unknown",
            "application_key",
            "application_secret",
            "consumer_key",
        );

        assert!(matches!(res, Err(OvhManagerError::EndpointNotFound(_))));
    }

    #[tokio::test]
    async fn time() {
        let client = OvhClient::new(
            "ovh-eu",
            "application_key",
            "application_secret",
            "consumer_key",
        )
        .unwrap();
        let res = client.get_server_time().await;

        assert!(res.is_ok())
    }
}
