use std::collections::BTreeSet;

use super::{
    item::SubscanModulePoolResultItem,
    statistics::{SubscanModulePoolStatistics, SubscanModuleStatistics},
};

/// Stores [`SubscanModulePool`](crate::pools::module::SubscanModulePool) results
#[derive(Clone, Debug, Default)]
pub struct SubscanModulePoolResult {
    /// Pool statistics, includes each module statistics
    /// and IP resolver statistics
    pub statistics: SubscanModulePoolStatistics,
    /// Subdomains that have been discovered
    pub items: BTreeSet<SubscanModulePoolResultItem>,
}

impl SubscanModulePoolResult {
    /// Add a single [`SubscanModuleStatistics`] into [`SubscanModulePoolResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::{
    ///     pool::SubscanModulePoolResult,
    ///     statistics::SubscanModuleStatistics,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result = SubscanModulePoolResult::default();
    ///     let stat = SubscanModuleStatistics::skipped("foo");
    ///
    ///     assert_eq!(result.statistics.module.len(), 0);
    ///
    ///     result.statistic(stat).await;
    ///
    ///     assert_eq!(result.statistics.module.len(), 1);
    /// }
    /// ```
    pub async fn statistic(&mut self, stats: SubscanModuleStatistics) {
        self.statistics.module(stats).await;
    }
}
