use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, TimeDelta, Utc};
use serde::Serialize;

use super::status::{SkipReason::SkippedByUser, SubscanModuleStatus};
use crate::utilities::serializers::td_to_seconds;

/// Subscan result statistics data type
pub type SubscanResultStatistics = HashMap<String, SubscanModuleStatistic>;

/// Stores single [`SubscanModule`](crate::types::core::SubscanModule) statistics
#[derive(Clone, Debug, Serialize)]
pub struct SubscanModuleStatistic {
    pub status: SubscanModuleStatus,
    pub count: usize,
    #[serde(with = "ts_seconds")]
    pub started_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub finished_at: DateTime<Utc>,
    #[serde(serialize_with = "td_to_seconds")]
    pub elapsed: TimeDelta,
}

impl Default for SubscanModuleStatistic {
    fn default() -> Self {
        Self {
            status: SubscanModuleStatus::Started,
            count: 0,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}

impl SubscanModuleStatistic {
    /// Create skipped module statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::{
    ///     statistics::SubscanModuleStatistic,
    ///     status::SkipReason::SkippedByUser,
    /// };
    ///
    /// let skipped = SubscanModuleStatistic::skipped();
    ///
    /// assert_eq!(skipped.status, SkippedByUser.into());
    /// assert_eq!(skipped.count, 0);
    /// assert_eq!(skipped.elapsed.num_seconds(), 0);
    /// ```
    pub fn skipped() -> Self {
        Self {
            status: SkippedByUser.into(),
            count: 0,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }

    pub async fn finish_with_status(&mut self, status: SubscanModuleStatus) {
        self.finished_at = Utc::now();
        self.status = status;
        self.elapsed = self.finished_at - self.started_at;
    }
}
