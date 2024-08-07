#[cfg(test)]
mod tests {
    use ovh_cloud_manager::client::OvhClient;
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
