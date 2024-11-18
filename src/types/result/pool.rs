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
    pub async fn statistic(&mut self, stats: SubscanModuleStatistics) {
        self.statistics.module(stats).await;
    }
}
