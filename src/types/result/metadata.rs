use crate::utilities::serializers::{dt_to_string_method, td_num_seconds_method};
use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;

/// [`ScanResult`](crate::types::result::scan::ScanResult) metadata struct definition
#[derive(Clone, Default, Serialize)]
pub struct ScanResultMetadata {
    /// Target domain address have been scanned
    pub target: String,
    /// Date and time the scan started as [`DateTime`]
    #[serde(serialize_with = "dt_to_string_method")]
    pub started_at: DateTime<Utc>,
    /// Date and time the scan finished as [`DateTime`]
    #[serde(serialize_with = "dt_to_string_method")]
    pub finished_at: DateTime<Utc>,
    /// Elapsed time during the scan
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
