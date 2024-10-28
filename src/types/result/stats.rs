use crate::{
    enums::module::SubscanModuleStatus,
    types::result::module::SubscanModuleResult,
    utils::serializers::{dt_to_string_method, td_num_seconds_method},
};
use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;

/// Stores single [`SubscanModule`](crate::types::core::SubscanModule) statistics
#[derive(Clone, Serialize)]
pub struct SubscanModuleStatistics {
    /// Module name
    pub module: String,
    /// Module last state
    pub status: SubscanModuleStatus,
    /// Count of discovered subdomains by module
    pub count: usize,
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

impl From<SubscanModuleResult> for SubscanModuleStatistics {
    fn from(result: SubscanModuleResult) -> Self {
        Self {
            module: result.module.clone(),
            status: result.status.clone(),
            count: result.subdomains.len(),
            started_at: result.started_at,
            finished_at: result.finished_at,
            elapsed: result.elapsed(),
        }
    }
}
