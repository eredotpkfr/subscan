use crate::{
    enums::module::SubscanModuleStatus,
    types::{core::Subdomain, result::stats::SubscanModuleStatistics},
};
use chrono::{DateTime, TimeDelta, Utc};
use std::collections::BTreeSet;

/// `Subscan` module result, it stores findings and module execution status
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct SubscanModuleResult {
    pub module: String,
    pub subdomains: BTreeSet<Subdomain>,
    pub status: SubscanModuleStatus,
    pub started_at: DateTime<Utc>,
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
    /// use subscan::types::result::module::SubscanModuleResult;
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
    /// use subscan::types::result::module::SubscanModuleResult;
    /// use subscan::enums::module::SubscanModuleStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///     let finished = result.with_status(SubscanModuleStatus::Finished).await;
    ///
    ///     assert!(finished.elapsed().subsec_nanos() > 0);
    /// }
    /// ```
    pub fn elapsed(&self) -> TimeDelta {
        self.finished_at - self.started_at
    }

    /// Get module stats as [`SubscanModuleStatistics`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::module::SubscanModuleResult;
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
    pub fn stats(&self) -> SubscanModuleStatistics {
        self.clone().into()
    }

    /// Update [`SubscanModuleResult::finished_at`] field and return itself
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::module::SubscanModuleResult;
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
