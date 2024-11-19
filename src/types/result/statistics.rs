use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;

use super::status::{SkipReason::SkippedByUser, SubscanModuleStatus};
use crate::{
    types::result::module::SubscanModuleResult,
    utilities::serializers::{dt_to_string_method, td_num_seconds_method},
};

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
    /// Create skipped module statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::{
    ///     statistics::SubscanModuleStatistics,
    ///     status::SkipReason::SkippedByUser,
    /// };
    ///
    /// let skipped = SubscanModuleStatistics::skipped("foo");
    ///
    /// assert_eq!(skipped.module, "foo");
    /// assert_eq!(skipped.status, SkippedByUser.into());
    /// assert_eq!(skipped.count, 0);
    /// assert_eq!(skipped.elapsed.num_seconds(), 0);
    /// ```
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
    /// Set [`started_at`](crate::types::result::statistics::ResolverStatistics::started_at) to [`Utc::now`]
    pub fn started(&mut self) -> DateTime<Utc> {
        self.started_at = Utc::now();
        self.started_at
    }

    /// Set [`finished_at`](crate::types::result::statistics::ResolverStatistics::finished_at) to [`Utc::now`]
    /// and calculate [`elapsed`](crate::types::result::statistics::ResolverStatistics::elapsed) value
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
    /// Add [`SubscanModuleStatistics`] into [`SubscanModulePoolStatistics`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::statistics::{
    ///     SubscanModulePoolStatistics,
    ///     SubscanModuleStatistics,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut pool_stats = SubscanModulePoolStatistics::default();
    ///     let module_stat = SubscanModuleStatistics::skipped("foo");
    ///
    ///     assert_eq!(pool_stats.module.len(), 0);
    ///
    ///     pool_stats.module(module_stat).await;
    ///
    ///     assert_eq!(pool_stats.module.len(), 1);
    /// }
    /// ```
    pub async fn module(&mut self, stats: SubscanModuleStatistics) {
        self.module.push(stats);
    }
}
