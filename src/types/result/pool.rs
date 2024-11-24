use std::collections::BTreeSet;

use super::{
    item::PoolResultItem,
    statistics::{PoolStatistics, SubscanModuleStatistic},
};

/// Stores [`SubscanModulePool`](crate::pools::module::SubscanModulePool) results
#[derive(Clone, Debug, Default)]
pub struct PoolResult {
    /// Pool statistics, includes each module statistics
    /// and IP resolver statistics
    pub statistics: PoolStatistics,
    /// Subdomains that have been discovered
    pub items: BTreeSet<PoolResultItem>,
}

impl PoolResult {
    /// Add a single [`SubscanModuleStatistic`] into [`PoolResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::{
    ///     pool::PoolResult,
    ///     statistics::SubscanModuleStatistic,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result = PoolResult::default();
    ///     let stat = SubscanModuleStatistic::skipped("foo");
    ///
    ///     assert_eq!(result.statistics.module.len(), 0);
    ///
    ///     result.statistic(stat).await;
    ///
    ///     assert_eq!(result.statistics.module.len(), 1);
    /// }
    /// ```
    pub async fn statistic(&mut self, stats: SubscanModuleStatistic) {
        self.statistics.module(stats).await;
    }
}
