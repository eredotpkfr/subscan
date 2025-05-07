use super::subscan::SubscanConfig;
use crate::enums::cache::CacheFilter;

#[derive(Clone, Default)]
pub struct PoolConfig {
    pub filter: CacheFilter,
    pub concurrency: u64,
    pub print: bool,
}

impl From<SubscanConfig> for PoolConfig {
    fn from(config: SubscanConfig) -> Self {
        Self {
            filter: config.filter,
            concurrency: config.concurrency,
            print: config.print,
        }
    }
}
