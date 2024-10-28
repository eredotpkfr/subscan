use crate::{
    enums::module::SubscanModuleStatus,
    types::{
        core::Subdomain,
        result::{metadata::SubscanScanResultMetadata, stats::SubscanModuleStatistics},
    },
};
use chrono::Utc;
use serde::Serialize;
use std::collections::BTreeSet;

/// `Subscan` scan result
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

impl From<&str> for SubscanScanResult {
    fn from(target: &str) -> Self {
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
    /// Update `finished_at`, `elapsed` and `total` fields and returns itself
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::SubscanScanResult;
    /// use std::collections::BTreeSet;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result: SubscanScanResult = "foo.com".into();
    ///
    ///     result.extend(BTreeSet::from_iter(["bar.foo.com".into()]));
    ///
    ///     let finished = result.clone().with_finished().await;
    ///
    ///     assert_eq!(finished.metadata.target, "foo.com");
    ///     assert_eq!(finished.total, 1);
    /// }
    /// ```
    pub async fn with_finished(mut self) -> Self {
        self.metadata.finished_at = Utc::now();
        self.metadata.elapsed = self.metadata.finished_at - self.metadata.started_at;
        self.total = self.results.len();

        self
    }

    /// Group modules by their statuses
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::SubscanScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result: SubscanScanResult = "foo.com".into();
    ///
    ///     result.add_status("one", SubscanModuleStatus::Started).await;
    ///     result.add_status("two", SkipReason::NotAuthenticated.into()).await;
    ///     result.add_status("three", SubscanModuleStatus::Finished).await;
    ///     result.add_status("four", SubscanModuleStatus::Failed("bar".into())).await;
    ///
    ///     assert_eq!(result.metadata.started.len(), 1);
    ///     assert_eq!(result.metadata.skipped.len(), 1);
    ///     assert_eq!(result.metadata.finished.len(), 1);
    ///     assert_eq!(result.metadata.failed.len(), 1);
    /// }
    /// ```
    pub async fn add_status(&mut self, module: &str, status: SubscanModuleStatus) -> bool {
        match status {
            SubscanModuleStatus::Started => self.metadata.started.insert(module.to_string()),
            SubscanModuleStatus::Skipped(_) => self.metadata.skipped.insert(module.to_string()),
            SubscanModuleStatus::Finished => self.metadata.finished.insert(module.to_string()),
            SubscanModuleStatus::Failed(_) => self.metadata.failed.insert(module.to_string()),
        }
    }

    /// Add module statistics on [`SubscanScanResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::SubscanScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    /// use subscan::types::result::module::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut scan_result: SubscanScanResult = "foo.com".into();
    ///     let module_result: SubscanModuleResult = "foo".into();
    ///
    ///     scan_result.add_statistic(module_result.into()).await;
    ///
    ///     assert_eq!(scan_result.statistics.len(), 1);
    /// }
    /// ```
    pub async fn add_statistic(&mut self, stats: SubscanModuleStatistics) {
        self.statistics.push(stats);
    }
}
