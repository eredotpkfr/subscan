use std::collections::BTreeSet;

use chrono::{DateTime, TimeDelta, Utc};

use super::status::SubscanModuleStatus;
use crate::{
    error::SubscanError,
    types::{core::Subdomain, result::statistics::SubscanModuleStatistics},
};

/// `Subscan` module result, it stores findings and module execution status
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
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
    /// use subscan::types::result::{
    ///     module::SubscanModuleResult,
    ///     status::SubscanModuleStatus
    /// };
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

    /// Updated status to [`SubscanModuleStatus::Finished`] and [`finished_at`](crate::types::result::module::SubscanModuleResult::finished_at)
    /// to [`Utc::now()`] and returns itself
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::{
    ///     module::SubscanModuleResult,
    ///     status::SubscanModuleStatus,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = SubscanModuleResult::default();
    ///
    ///     assert_eq!(result.status, SubscanModuleStatus::Started);
    ///     assert_eq!(
    ///         result.with_finished().await.status,
    ///         SubscanModuleStatus::Finished
    ///     );
    /// }
    /// ```
    pub async fn with_finished(mut self) -> Self {
        self.status = SubscanModuleStatus::Finished;
        self.finished_at = Utc::now();
        self
    }

    /// Make a graceful exit if any subdomain available in result, returns
    /// [`SubscanError::ModuleErrorWithResult`] error type
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::{
    ///     module::SubscanModuleResult,
    ///     status::SubscanModuleStatus,
    /// };
    /// use std::collections::BTreeSet;
    /// use subscan::error::{SubscanError, ModuleErrorKind::JSONExtract};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result = SubscanModuleResult::default();
    ///     let graceful = result.graceful_exit().await(JSONExtract.into());
    ///
    ///     assert_eq!(graceful, JSONExtract.into());
    ///     assert_eq!(graceful.status(), SubscanModuleStatus::Failed(JSONExtract.into()));
    ///
    ///     let mut result = SubscanModuleResult::default();
    ///     let mut expected = result.clone();
    ///
    ///     result.extend(BTreeSet::from_iter(["bar.foo.com".into()]));
    ///
    ///     expected.extend(BTreeSet::from_iter(["bar.foo.com".into()]));
    ///     expected.status = SubscanModuleStatus::FailedWithResult;
    ///
    ///     let graceful = result.graceful_exit().await(JSONExtract.into());
    ///
    ///     assert_eq!(graceful, SubscanError::ModuleErrorWithResult(expected));
    ///
    ///     if let SubscanError::ModuleErrorWithResult(inner) = graceful {
    ///         assert_eq!(inner.subdomains, ["bar.foo.com".into()].into());
    ///     }
    /// }
    /// ```
    pub async fn graceful_exit(&mut self) -> impl Fn(SubscanError) -> SubscanError + '_ {
        |err| {
            let mut res = self.clone();

            if !self.subdomains.is_empty() {
                res.status = SubscanModuleStatus::FailedWithResult;
                SubscanError::ModuleErrorWithResult(res)
            } else {
                res.status = err.status();
                err
            }
        }
    }
}
