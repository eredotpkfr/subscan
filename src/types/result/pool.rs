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
