use crate::{
    error::{SkipReason::SkippedByUser, SubscanModuleStatus},
    types::result::module::SubscanModuleResult,
    utilities::serializers::{dt_to_string_method, td_num_seconds_method},
};
use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;

/// Alias for [`SubscanModulePoolStatistics`]
pub type ScanResultStatistics = SubscanModulePoolStatistics;

/// Stores single [`SubscanModule`](crate::types::core::SubscanModule) statistics
#[derive(Clone, Debug, Serialize)]
pub struct SubscanModuleStatistics {
    pub module: String,
    pub status: SubscanModuleStatus,
    pub count: usize,
    #[serde(serialize_with = "dt_to_string_method")]
    pub started_at: DateTime<Utc>,
    #[serde(serialize_with = "dt_to_string_method")]
    pub finished_at: DateTime<Utc>,
    #[serde(serialize_with = "td_num_seconds_method")]
    pub elapsed: TimeDelta,
}

impl SubscanModuleStatistics {
    pub fn skipped(module: &str) -> Self {
        Self {
            module: module.into(),
            status: SkippedByUser.into(),
            count: 0,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }

    pub async fn log(&self) {
        self.status.log(&self.module);
    }
}

impl From<SubscanModuleResult> for SubscanModuleStatistics {
    fn from(result: SubscanModuleResult) -> Self {
        Self {
            module: result.module.clone(),
            status: result.clone().status,
            count: result.subdomains.len(),
            started_at: result.started_at,
            finished_at: result.finished_at,
            elapsed: result.elapsed(),
        }
    }
}

/// Stores IP address resolver component statistics like a start time, end time
/// or elapsed time during the resolving process
#[derive(Clone, Debug, Serialize)]
pub struct ResolverStatistics {
    #[serde(serialize_with = "dt_to_string_method")]
    pub started_at: DateTime<Utc>,
    #[serde(serialize_with = "dt_to_string_method")]
    pub finished_at: DateTime<Utc>,
    #[serde(serialize_with = "td_num_seconds_method")]
    pub elapsed: TimeDelta,
}

impl Default for ResolverStatistics {
    fn default() -> Self {
        Self {
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}

impl ResolverStatistics {
    /// Set [`started_at`](crate::types::result::stats::ResolverStatistics::started_at) to [`Utc::now`]
    pub fn started(&mut self) -> DateTime<Utc> {
        self.started_at = Utc::now();
        self.started_at
    }

    /// Set [`finished_at`](crate::types::result::stats::ResolverStatistics::finished_at) to [`Utc::now`]
    /// and calculate [`elapsed`](crate::types::result::stats::ResolverStatistics::elapsed) value
    pub fn finished(&mut self) -> DateTime<Utc> {
        self.finished_at = Utc::now();
        self.elapsed = self.finished_at - self.started_at;
        self.finished_at
    }
}

/// Stores [`SubscanModulePool`](crate::pools::module::SubscanModulePool) statistics
#[derive(Clone, Debug, Default, Serialize)]
pub struct SubscanModulePoolStatistics {
    pub module: Vec<SubscanModuleStatistics>,
    pub resolve: ResolverStatistics,
}

impl SubscanModulePoolStatistics {
    pub async fn module(&mut self, stats: SubscanModuleStatistics) {
        self.module.push(stats);
    }
}
