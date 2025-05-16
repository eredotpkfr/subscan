use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    sync::RwLock,
};

use super::subscan::SubscanConfig;
use crate::enums::cache::CacheFilter;

#[derive(Clone, Default)]
pub struct PoolConfig {
    pub concurrency: u64,
    pub filter: CacheFilter,
    pub print: bool,
    pub stream: Option<PathBuf>,
}

impl From<SubscanConfig> for PoolConfig {
    /// Create [`PoolConfig`] from [`SubscanConfig`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::{pool::PoolConfig, subscan::SubscanConfig};
    ///
    /// let sconfig = SubscanConfig::default();
    /// let config = PoolConfig::from(sconfig.clone());
    ///
    /// assert_eq!(sconfig.concurrency, config.concurrency);
    /// assert_eq!(sconfig.filter, config.filter);
    /// assert_eq!(sconfig.print, config.print);
    /// assert_eq!(sconfig.stream, config.stream);
    /// ```
    fn from(config: SubscanConfig) -> Self {
        Self {
            concurrency: config.concurrency,
            filter: config.filter,
            print: config.print,
            stream: config.stream,
        }
    }
}

impl PoolConfig {
    /// Get stream file descriptor
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::{pool::PoolConfig, subscan::SubscanConfig};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sconfig = SubscanConfig::default();
    ///     let config = PoolConfig::from(sconfig.clone());
    ///
    ///     assert!(config.get_stream_file().await.is_none());
    /// }
    /// ```
    pub async fn get_stream_file(&self) -> Option<RwLock<File>> {
        self.stream.as_ref().map(|path| {
            RwLock::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .read(true)
                    .truncate(false)
                    .open(path)
                    .unwrap_or_else(|_| panic!("Cannot create {} file!", path.to_str().unwrap())),
            )
        })
    }
}
