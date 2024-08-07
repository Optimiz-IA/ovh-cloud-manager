use serde::{Deserialize, Serialize};

use crate::schemas::common::{NameObject, UrlObject};

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvVars {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub cpu: Option<u64>,
    pub ephemeral_storage: Option<u64>,
    pub flavor: Option<String>,
    pub gpu: Option<u64>,
    pub gpu_brand: Option<String>,
    pub gpu_memory: Option<u64>,
    pub gpu_model: Option<String>,
    pub memory: Option<u64>,
    pub private_network: Option<u64>,
    pub public_network: Option<u64>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DataStore {
    pub alias: String,
    pub archive: Option<String>,
    pub container: String,
    pub internal: Option<bool>,
    pub prefix: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PrivateSwift {
    pub archive: Option<String>,
    pub container: String,
    pub internal: Option<bool>,
    pub prefix: Option<String>,
    pub region: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub cache: bool,
    pub container: Option<String>,
    pub data_store: Option<DataStore>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub mount_path: String,
    pub permission: String,
    pub prefix: Option<String>,
    pub private_swift: Option<PrivateSwift>,
    pub public_git: Option<UrlObject>,
    pub public_swift: Option<UrlObject>,
    pub region: Option<String>,
    pub standalone: Option<NameObject>,
    pub target_data_store: Option<DataStore>,
    pub target_private_swift: Option<PrivateSwift>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    pub command: Vec<String>,
    pub default_http_port: Option<u64>,
    pub env_vars: Vec<EnvVars>,
    pub grpc_port: Option<u64>,
    pub image: String,
    pub labels: Option<serde_json::Value>,
    pub name: String,
    pub partner_id: Option<String>,
    pub read_user: Option<String>,
    pub region: String,
    pub resources: Resources,
    #[serde(skip_serializing)]
    pub shutdown: Option<String>,
    pub ssh_public_keys: Option<Vec<String>>,
    pub timeout: Option<u64>,
    pub timeout_auto_restart: Option<bool>,
    pub unsecure_http: Option<bool>,
    pub volumes: Vec<Volume>,
}
