use crate::{
    error::{SubscanError, SubscanModuleStatus},
    types::{core::Subdomain, result::stats::SubscanModuleStatistics},
};
use chrono::{DateTime, TimeDelta, Utc};
use std::collections::BTreeSet;

/// `Subscan` module result, it stores findings and module execution status
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct SubscanModuleResult {
    pub module: String,
    pub status: SubscanModuleStatus,
    pub subdomains: BTreeSet<Subdomain>,
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
    /// Get elapsed time during the module execution
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::module::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///     let finished = result.with_finished().await;
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
    /// use subscan::error::SubscanModuleStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///     let finished = result.with_finished().await;
    ///
    ///     assert_eq!(
    ///         finished.stats().await.status,
    ///         SubscanModuleStatus::Finished
    ///     );
    /// }
    /// ```
    pub async fn stats(&self) -> SubscanModuleStatistics {
        self.clone().into()
    }

    pub async fn with_finished(mut self) -> Self {
        self.status = SubscanModuleStatus::Finished;
        self.finished_at = Utc::now();
        self
    }

    pub async fn graceful_exit(&mut self) -> impl Fn(SubscanError) -> SubscanError + '_ {
        |err| {
            if !self.subdomains.is_empty() {
                SubscanError::ModuleErrorWithResult(self.clone())
            } else {
                err
            }
        }
    }
}
