use super::{item::SubscanResultItem, statistics::SubscanModuleStatistic};
use crate::types::result::{item::SubscanResultItems, statistics::SubscanResultStatistics};

/// Stores [`SubscanModulePool`](crate::pools::module::SubscanModulePool) results
#[derive(Clone, Debug, Default)]
pub struct PoolResult {
    /// Pool statistics, includes each module statistics
    /// and IP resolver statistics
    pub statistics: SubscanResultStatistics,
    /// Subdomains that have been discovered
    pub items: SubscanResultItems,
}

impl PoolResult {
    pub async fn insert(&mut self, module: &str, subdomain: SubscanResultItem) -> bool {
        let defaults = SubscanModuleStatistic::default();
        let inserted = self.items.insert(subdomain);

        let stats = self.statistics.entry(module.to_owned()).or_insert(defaults);

        if inserted {
            stats.count += 1;
        }

        inserted
    }
}
