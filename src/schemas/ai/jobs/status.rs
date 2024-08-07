use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schemas::common::Info;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub completed: u64,
    pub created_at: DateTime<Utc>,
    pub deleted: u64,
    pub direction: String,
    pub eta: Option<u64>,
    pub failed: u64,
    pub id: String,
    pub info: String,
    pub processed: u64,
    pub skipped: u64,
    pub state: String,
    pub total: u64,
    pub transferred_bytes: u64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSyncStatus {
    pub ended_at: Option<DateTime<Utc>>,
    pub info: Info,
    pub progress: Vec<Progress>,
    pub queued_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataSyncSpec {
    pub direction: String,
    pub manual: bool,
    pub volume: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSync {
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub spec: DataSyncSpec,
    pub status: DataSyncStatus,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryState {
    pub date: DateTime<Utc>,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatusVolume {
    pub id: String,
    pub mount_path: String,
    pub user_volume_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    pub data_sync: Vec<DataSync>,
    pub duration: Option<u64>,
    pub exit_code: Option<u64>,
    pub external_ip: Option<String>,
    pub finalized_at: Option<DateTime<Utc>>,
    pub grpc_address: Option<String>,
    pub history: Vec<HistoryState>,
    pub info: Info,
    pub info_url: Option<String>,
    pub initializing_at: Option<DateTime<Utc>>,
    pub ip: Option<String>,
    pub last_transition_date: Option<DateTime<Utc>>,
    pub monitoring_url: Option<String>,
    pub queued_at: Option<DateTime<Utc>>,
    pub ssh_url: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub state: Option<String>,
    pub stopped_at: Option<DateTime<Utc>>,
    pub timeout_at: Option<DateTime<Utc>>,
    pub url: Option<String>,
    pub volumes: Vec<JobStatusVolume>,
}
