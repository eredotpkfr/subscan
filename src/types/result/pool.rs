use std::collections::BTreeSet;

use super::{item::SubscanModulePoolResultItem, stats::SubscanModulePoolStatistics};

/// Stores [`SubscanModulePool`](crate::pools::module::SubscanModulePool) results
#[derive(Clone, Debug, Default)]
pub struct SubscanModulePoolResult {
    /// Pool statistics, includes each module statistics
    /// and IP resolver statistics
    pub statistics: SubscanModulePoolStatistics,
    /// Subdomains that have been discovered
    pub results: BTreeSet<SubscanModulePoolResultItem>,
}
