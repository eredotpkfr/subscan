use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;

use crate::utilities::serializers::{dt_to_string_method, td_num_seconds_method};

/// [`ScanResult`](crate::types::result::scan::ScanResult) metadata struct definition
#[derive(Clone, Default, Serialize)]
pub struct ScanResultMetadata {
    pub target: String,
    #[serde(serialize_with = "dt_to_string_method")]
    pub started_at: DateTime<Utc>,
    #[serde(serialize_with = "dt_to_string_method")]
    pub finished_at: DateTime<Utc>,
    #[serde(serialize_with = "td_num_seconds_method")]
    pub elapsed: TimeDelta,
}

impl From<&str> for ScanResultMetadata {
    fn from(target: &str) -> Self {
        Self {
            target: target.to_string(),
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}
