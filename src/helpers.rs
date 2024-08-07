use crate::schemas::ai::jobs::spec::{DataStore, JobSpec, Resources, Volume};

pub fn create_volume(
    container: &str,
    alias: &str,
    prefix: Option<&str>,
    mount_path: &str,
    permission: &str,
) -> Volume {
    Volume {
        data_store: Some(DataStore {
            alias: alias.to_string(),
            container: container.to_string(),
            prefix: prefix.map(str::to_string),
            ..Default::default()
        }),
        mount_path: mount_path.to_string(),
        permission: permission.to_string(),
        cache: true,
        ..Default::default()
    }
}

pub fn create_job_spec(
    region: &str,
    command: Vec<String>,
    image: &str,
    volumes: Vec<Volume>,
    labels: Option<serde_json::Value>,
    flavor: Option<&str>,
) -> JobSpec {
    JobSpec {
        command,
        volumes,
        image: image.to_string(),
        region: region.to_string(),
        resources: Resources {
            flavor: flavor.map(str::to_string),
            ..Default::default()
        },
        labels,
        ..Default::default()
    }
}
