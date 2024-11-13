use std::collections::BTreeSet;

use super::{item::SubscanModulePoolResultItem, stats::SubscanModulePoolStatistics};

#[derive(Clone, Debug, Default)]
pub struct SubscanModulePoolResult {
    /// Pool statistics, includes each module statistics
    pub statistics: SubscanModulePoolStatistics,
    /// Subdomains that have been discovered
    pub results: BTreeSet<SubscanModulePoolResultItem>,
}
