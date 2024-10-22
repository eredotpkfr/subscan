/// In-memory cache to store all modules
pub mod cache;
/// Includes CLI components
pub mod cli;
/// Project configuration utils
pub mod config;
/// Enumerations and project type definitions
pub mod enums;
/// Data extractors like
/// [`extractors::regex`], [`extractors::html`], etc.
pub mod extractors;
/// Trait implementations
pub mod interfaces;
/// All modules listed under this module, core components for subscan
pub mod modules;
/// HTTP requesters listed under this module
/// like [`requesters::chrome`], [`requesters::client`], etc.
pub mod requesters;
/// Project core type definitions
pub mod types;
/// Utilities for the handle different stuff things
pub mod utils;

use crate::{
    cache::CacheManager, cli::Cli, interfaces::module::SubscanModuleInterface,
    types::config::SubscanConfig,
};
use enums::SubscanModuleDispatcher;
use tokio::sync::{Mutex, OnceCell};

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

    pub async fn module(&self, name: &str) -> Option<&Mutex<SubscanModuleDispatcher>> {
        self.manager.module(name).await
    }

    pub async fn run(&self, name: &str, domain: &str) {
        self.init().await;

        let module = self.module(name).await.expect("Not found!");

        for res in module.lock().await.run(domain).await {
            println!("{}", res);
        }
    }
}
