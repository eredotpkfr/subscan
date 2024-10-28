use crate::{
    enums::module::SubscanModuleStatus,
    types::core::Subdomain,
    utils::serializers::{dt_to_string_method, td_num_seconds_method},
};
use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;
use std::collections::BTreeSet;

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

impl From<String> for SubscanScanResultMetadata {
    fn from(target: String) -> Self {
        Self {
            target,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Serialize)]
pub struct SubscanScanResult {
    /// Scan metadata
    pub metadata: SubscanScanResultMetadata,
    /// Module statistics
    pub statistics: Vec<SubscanModuleStatistics>,
    /// Subscans that have been discovered
    pub results: BTreeSet<Subdomain>,
    /// Total count of discovered subdomains
    pub total: usize,
}

impl From<String> for SubscanScanResult {
    fn from(target: String) -> Self {
        Self {
            metadata: target.into(),
            ..Default::default()
        }
    }
}

impl Extend<Subdomain> for SubscanScanResult {
    fn extend<T: IntoIterator<Item = Subdomain>>(&mut self, iter: T) {
        self.results.extend(iter);
    }
}

impl SubscanScanResult {
    pub async fn with_finished(mut self) -> Self {
        self.metadata.finished_at = Utc::now();
        self.metadata.elapsed = self.metadata.finished_at - self.metadata.started_at;
        self.total = self.results.len();

        self
    }

    pub async fn add_status(&mut self, module: &str, status: SubscanModuleStatus) -> bool {
        match status {
            SubscanModuleStatus::Started => self.metadata.started.insert(module.to_string()),
            SubscanModuleStatus::Skipped(_) => self.metadata.skipped.insert(module.to_string()),
            SubscanModuleStatus::Finished => self.metadata.finished.insert(module.to_string()),
            SubscanModuleStatus::Failed(_) => self.metadata.failed.insert(module.to_string()),
        }
    }

    pub async fn add_statistic(&mut self, stats: SubscanModuleStatistics) {
        self.statistics.push(stats);
    }
}

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

/// `Subscan` module result, it stores findings and module execution status
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct SubscanModuleResult {
    /// Module name
    pub module: String,
    /// Discovered subdomain list
    pub subdomains: BTreeSet<Subdomain>,
    /// Last state of module, see the [`SubscanModuleStatus`] for variants
    pub status: SubscanModuleStatus,
    /// Date and time the module started as [`DateTime`]
    pub started_at: DateTime<Utc>,
    /// Date and time the module finished as [`DateTime`]
    pub finished_at: DateTime<Utc>,
}

impl Extend<Subdomain> for SubscanModuleResult {
    fn extend<T: IntoIterator<Item = Subdomain>>(&mut self, iter: T) {
        self.subdomains.extend(iter)
    }
}

impl From<&str> for SubscanModuleResult {
    fn from(module: &str) -> Self {
        Self {
            module: module.to_string(),
            started_at: Utc::now(),
            finished_at: Utc::now(),
            ..Default::default()
        }
    }
}

impl SubscanModuleResult {
    /// Set module status with a new status
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    /// use subscan::enums::module::SubscanModuleStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result = SubscanModuleResult::default();
    ///
    ///     assert_eq!(result.status, SubscanModuleStatus::Started);
    ///
    ///     result.set_status(SubscanModuleStatus::Finished).await;
    ///
    ///     assert_eq!(result.status, SubscanModuleStatus::Finished);
    /// }
    /// ```
    pub async fn set_status(&mut self, status: SubscanModuleStatus) {
        self.status = status
    }

    /// Get elapsed time during the module execution
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    /// use subscan::enums::module::SubscanModuleStatus;
    ///
    /// let result = SubscanModuleResult::default();
    /// let finished = result.with_status(SubscanModuleStatus::Finished).await;
    ///
    /// assert!(finished.elapsed().await.subsec_nanos() > 0);
    /// ```
    pub fn elapsed(&self) -> TimeDelta {
        self.finished_at - self.started_at
    }

    /// Get module stats as [`SubscanModuleStatistics`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    /// use subscan::enums::module::SubscanModuleStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///     let finished = result.with_status(SubscanModuleStatus::Finished).await;
    ///
    ///     assert_eq!(finished.stats().status, SubscanModuleStatus::Finished);
    /// }
    /// ```
    pub fn stats(self) -> SubscanModuleStatistics {
        self.into()
    }

    /// Update [`SubscanModuleResult::finished_at`] field and return itself
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    /// use subscan::enums::module::SubscanModuleStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///     let finished = result.with_status(SubscanModuleStatus::Finished).await;
    ///
    ///     assert_eq!(finished.status, SubscanModuleStatus::Finished);
    ///     assert!(finished.finished_at > finished.started_at);
    /// }
    /// ```
    pub async fn with_status(mut self, status: SubscanModuleStatus) -> Self {
        self.status = status;
        self.finished_at = Utc::now();
        self
    }
}
