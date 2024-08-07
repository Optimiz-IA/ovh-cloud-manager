pub mod capabilities;
pub mod data;
pub mod jobs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ResourceAIQuota {
    pub cpu: u64,
    pub gpu: u64,
}

#[derive(Debug, Deserialize)]
pub struct AIQuota {
    pub storage: u64,
    pub resources: ResourceAIQuota,
}
