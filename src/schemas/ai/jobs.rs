pub mod spec;
pub mod status;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AIJob {
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub spec: spec::JobSpec,
    pub status: status::JobStatus,
    pub updated_at: DateTime<Utc>,
    pub user: String,
}
