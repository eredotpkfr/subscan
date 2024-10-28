use crate::{enums::SubscanModuleStatus, types::core::Subdomain};
use chrono::{DateTime, TimeDelta, Utc};
use std::collections::BTreeSet;

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
    /// Update [`SubscanModuleResult::finished_at`] field and return itself
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    /// use subscan::enums::SubscanModuleStatus;
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

impl SubscanModuleResult {
    /// Returns count of subdomains that enumerated by module
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///
    ///     assert_eq!(result.len().await, 0);
    /// }
    /// ```
    pub async fn len(&self) -> usize {
        self.subdomains.len()
    }

    /// Returns [`true`] if empty otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///
    ///     assert!(result.is_empty().await);
    /// }
    /// ```
    pub async fn is_empty(&self) -> bool {
        self.subdomains.is_empty()
    }

    /// Set module status with a new status
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::SubscanModuleResult;
    /// use subscan::enums::SubscanModuleStatus;
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
    /// use subscan::enums::SubscanModuleStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///     let finished = result.with_status(SubscanModuleStatus::Finished).await;
    ///
    ///     assert!(finished.elapsed().await.subsec_nanos() > 0);
    /// }
    /// ```
    pub async fn elapsed(&self) -> TimeDelta {
        self.finished_at - self.started_at
    }
}
