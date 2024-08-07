use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInformation {
    pub gpu_brand: String,
    pub gpu_memory: u64,
    pub gpu_model: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesPerUnit {
    pub cpu: u64,
    pub ephemeral_storage: u64,
    pub memory: u64,
    pub private_network: u64,
    pub public_network: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AIFlavor {
    pub default: bool,
    pub description: String,
    pub gpu_information: Option<GpuInformation>,
    pub id: String,
    pub max: u64,
    pub resources_per_unit: ResourcesPerUnit,
    pub r#type: String,
}
