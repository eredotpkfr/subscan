use std::{
    fs::File,
    path::PathBuf,
    sync::{OnceLock, RwLock},
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

static STREAM_FILE: OnceLock<Option<RwLock<File>>> = OnceLock::new();

impl From<SubscanConfig> for PoolConfig {
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
    pub async fn get_stream_file(&self) -> &Option<RwLock<File>> {
        let inner =
            || {
                self.stream.as_ref().map(|path| {
                    RwLock::new(File::create(path).unwrap_or_else(|_| {
                        panic!("Cannot create {} file!", path.to_str().unwrap())
                    }))
                })
            };

        STREAM_FILE.get_or_init(inner)
    }
}
