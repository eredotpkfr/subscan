#![forbid(unsafe_code)]

/// In-memory cache to store all modules
pub mod cache;
/// Includes CLI components
pub mod cli;
/// Project constants
pub mod constants;
/// Enumerations and project type definitions
pub mod enums;
/// Subscan error type
pub mod error;
/// Data extractors like
/// [`extractors::regex`], [`extractors::html`], etc.
pub mod extractors;
/// Trait implementations
pub mod interfaces;
/// Logger utilities
pub mod logger;
/// All modules listed under this module, core components for subscan
pub mod modules;
/// `Subscan` worker pool definitions, allows to run modules as asynchronously
pub mod pools;
/// HTTP requesters listed under this module
/// like [`requesters::chrome`], [`requesters::client`], etc.
pub mod requesters;
/// IP address resolver component
pub mod resolver;
/// Project core type definitions
pub mod types;
/// Utilities for the handle different stuff things
pub mod utilities;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use constants::LOG_TIME_FORMAT;
use tokio::sync::OnceCell;

use crate::{
    cache::CacheManager,
    cli::Cli,
    interfaces::module::SubscanModuleInterface,
    pools::{brute::SubscanBrutePool, module::SubscanModulePool},
    types::{config::subscan::SubscanConfig, core::SubscanModule, result::subscan::SubscanResult},
};

static INIT: OnceCell<()> = OnceCell::const_new();

/// Main [`Subscan`] object definition
#[derive(Default)]
pub struct Subscan {
    /// Subscan configurations
    pub config: SubscanConfig,
    /// Cache manager instance to manage modules cache
    pub manager: CacheManager,
}

impl From<Cli> for Subscan {
    fn from(cli: Cli) -> Self {
        Self {
            config: cli.into(),
            manager: CacheManager::default(),
        }
    }
}

impl From<SubscanConfig> for Subscan {
    fn from(config: SubscanConfig) -> Self {
        Self {
            config,
            manager: CacheManager::default(),
        }
    }
}

impl Subscan {
    pub fn new(config: SubscanConfig) -> Self {
        Self {
            config,
            manager: CacheManager::default(),
        }
    }

    async fn init(&self) {
        let rconfig = self.config.clone().into();
        let inner = || async { self.manager.configure(rconfig).await };

        INIT.get_or_init(inner).await;
    }

    pub async fn module(&self, name: &str) -> &SubscanModule {
        self.manager.module(name).await.expect("Module not found!")
    }

    pub async fn modules(&self) -> &Vec<SubscanModule> {
        self.manager.modules().await
    }

    pub async fn scan(&self, domain: &str) -> SubscanResult {
        self.init().await;

        let mut result: SubscanResult = domain.into();

        let time = result.metadata.started_at.format(LOG_TIME_FORMAT);
        let pool = SubscanModulePool::from(domain, self.config.clone());

        log::info!("Started scan on {} ({})", domain, time);

        for module in self.modules().await {
            pool.clone().submit(module.clone()).await;
        }

        pool.clone().start(self.config.concurrency).await;

        result.update_with_pool_result(pool.result().await).await;
        result.with_finished().await
    }

    pub async fn run(&self, name: &str, domain: &str) -> SubscanResult {
        self.init().await;

        let mut result: SubscanResult = domain.into();

        let time = result.metadata.started_at.format(LOG_TIME_FORMAT);
        let pool = SubscanModulePool::from(domain, self.config.clone());
        let module = self.module(name).await;

        log::info!(
            "Running {} module on {} ({})",
            module.lock().await.name().await,
            domain,
            time
        );

        pool.clone().submit(module.clone()).await;
        pool.clone().start(1).await;

        result.update_with_pool_result(pool.result().await).await;
        result.with_finished().await
    }

    pub async fn brute(&self, domain: &str) -> SubscanResult {
        let mut result: SubscanResult = domain.into();

        let time = result.metadata.started_at.format(LOG_TIME_FORMAT);
        let pool = SubscanBrutePool::from(domain, self.config.clone());

        let concurrency = self.config.resolver.concurrency;
        let wordlist = self.config.wordlist.clone();
        let file = File::open(wordlist.expect("Wordlist must be specified!"));

        log::info!("Started brute force attack on {} ({})", domain, time);

        let reader = BufReader::new(file.expect("Cannot read wordlist!"));

        for subdomain in reader.lines().map_while(Result::ok) {
            pool.clone().submit(subdomain).await;
        }

        pool.clone().spawn_bruters(concurrency).await;
        pool.clone().join().await;

        result.update_with_pool_result(pool.result().await).await;
        result.with_finished().await
    }
}
