use crate::utilities::serializers::{dt_to_string_method, td_num_seconds_method};
use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;
use std::collections::BTreeSet;

/// [`SubscanScanResult`](crate::types::result::scan::SubscanScanResult) metadata struct definition
#[derive(Clone, Default, Serialize)]
pub struct SubscanScanResultMetadata {
    /// Target domain address have been scanned
    pub target: String,
    /// Modules that last state is started
    pub started: BTreeSet<String>,
    /// Finished modules list
    pub finished: BTreeSet<String>,
    /// Failed modules list
    pub failed: BTreeSet<String>,
    /// Skipped modules list
    pub skipped: BTreeSet<String>,
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

impl From<&str> for SubscanScanResultMetadata {
    fn from(target: &str) -> Self {
        Self {
            target: target.to_string(),
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
            ..Default::default()
        }
    }
}
