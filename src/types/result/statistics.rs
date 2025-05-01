use std::collections::HashMap;

use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;

use super::status::{SkipReason::SkippedByUser, SubscanModuleStatus};
use crate::utilities::serializers::{dt_to_string_method, td_num_seconds_method};

/// Subscan result statistics data type
pub type SubscanResultStatistics = HashMap<String, SubscanModuleStatistic>;

/// Stores single [`SubscanModule`](crate::types::core::SubscanModule) statistics
#[derive(Clone, Debug, Serialize)]
pub struct SubscanModuleStatistic {
    pub status: SubscanModuleStatus,
    pub count: usize,
    #[serde(serialize_with = "dt_to_string_method")]
    pub started_at: DateTime<Utc>,
    #[serde(serialize_with = "dt_to_string_method")]
    pub finished_at: DateTime<Utc>,
    #[serde(serialize_with = "td_num_seconds_method")]
    pub elapsed: TimeDelta,
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
}
