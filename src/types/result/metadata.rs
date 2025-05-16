use chrono::{serde::ts_seconds, DateTime, TimeDelta, Utc};
use serde::Serialize;

use crate::utilities::serializers::td_to_seconds;

/// [`SubscanResult`](crate::types::result::subscan::SubscanResult) metadata struct definition
#[derive(Clone, Debug, Serialize)]
pub struct SubscanResultMetadata {
    pub target: String,
    #[serde(with = "ts_seconds")]
    pub started_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub finished_at: DateTime<Utc>,
    #[serde(serialize_with = "td_to_seconds")]
    pub elapsed: TimeDelta,
}

impl Default for SubscanResultMetadata {
    fn default() -> Self {
        Self {
            target: String::new(),
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}

impl From<&str> for SubscanResultMetadata {
    fn from(target: &str) -> Self {
        Self {
            target: target.to_string(),
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}
