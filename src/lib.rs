/// In-memory cache to store all modules
pub mod cache;
/// Includes CLI components
pub mod cli;
/// Project constants
pub mod constants;
/// Enumerations and project type definitions
pub mod enums;
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
/// Project core type definitions
pub mod types;
/// Utilities for the handle different stuff things
pub mod utilities;

use crate::{
    cache::CacheManager, cli::Cli, enums::module::SkipReason::SkippedByUser,
    interfaces::module::SubscanModuleInterface, pools::module::SubscanModulePool,
    types::config::subscan::SubscanConfig, types::core::SubscanModule,
};
use tokio::sync::OnceCell;
use types::result::scan::ScanResult;

static INIT: OnceCell<()> = OnceCell::const_new();

/// Main `Subscan` object definition
#[derive(Default)]
pub struct Subscan {
    pub config: SubscanConfig,
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

    pub async fn scan(&self, domain: &str) -> ScanResult {
        self.init().await;

        let mut result: ScanResult = domain.into();

        let time = result.metadata.started_at.format("%H:%M:%S %Z");
        let pool = SubscanModulePool::new(domain.to_string(), self.config.resolver.clone());

        log::info!("Started scan on {} ({})", domain, time);

        for module in self.modules().await {
            let binding = module.lock().await;
            let name = binding.name().await;

            if !self.config.filter.is_filtered(name).await {
                pool.clone().submit(module.clone()).await;
            } else {
                result.add_status(name, &SkippedByUser.into()).await;
                utilities::log::status(name, SkippedByUser.into()).await;
            }
        }

        pool.clone().start(self.config.concurrency).await;

        result.update_with_pool_result(pool.result().await).await;
        result.with_finished().await
    }

    pub async fn run(&self, name: &str, domain: &str) -> ScanResult {
        self.init().await;

        let mut result: ScanResult = domain.into();

        let time = result.metadata.started_at.format("%H:%M:%S %Z");
        let pool = SubscanModulePool::new(domain.to_string(), self.config.resolver.clone());
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
}
