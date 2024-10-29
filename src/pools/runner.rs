use colored::Colorize;
use flume::{Receiver, Sender};
use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    enums::module::SubscanModuleStatus,
    interfaces::module::SubscanModuleInterface,
    types::{
        core::{SubscanModule, UnboundedFlumeChannel},
        result::module::SubscanModuleResult,
    },
};
use std::{collections::BTreeSet, sync::Arc};

/// Container for input channel, stores receiver and sender instances
pub struct SubscanModuleRunnerPoolInputChannel {
    pub tx: Sender<SubscanModule>,
    pub rx: Receiver<SubscanModule>,
}

impl From<UnboundedFlumeChannel> for SubscanModuleRunnerPoolInputChannel {
    fn from(channel: UnboundedFlumeChannel) -> Self {
        Self {
            tx: channel.0,
            rx: channel.1,
        }
    }
}

/// Runner pool to run multiple [`SubscanModuleRunnerPool::runner`]
pub struct SubscanModuleRunnerPool {
    domain: String,
    results: Mutex<BTreeSet<SubscanModuleResult>>,
    runners: Mutex<JoinSet<()>>,
    input: SubscanModuleRunnerPoolInputChannel,
}

impl SubscanModuleRunnerPool {
    pub fn new(domain: String) -> Arc<Self> {
        let runners = Mutex::new(JoinSet::new());
        let results = Mutex::new(BTreeSet::new());

        let input = flume::unbounded::<SubscanModule>().into();

        Arc::new(Self {
            domain,
            results,
            runners,
            input,
        })
    }

    /// [`SubscanModule`] runner method, simply calls `.run(` method
    pub async fn runner(self: Arc<Self>) {
        while let Ok(module) = self.input.rx.try_recv() {
            let mut module = module.lock().await;

            let result = module.run(&self.domain).await;
            let (name, status) = (module.name().await, result.status.with_reason().await);

            self.results.lock().await.insert(result.clone());

            match result.status {
                SubscanModuleStatus::Started => {
                    log::info!("{:.<25}{:.>35}", name.white(), status.white())
                }
                SubscanModuleStatus::Finished => {
                    log::info!("{:.<25}{:.>35}", name.white(), status.white())
                }
                SubscanModuleStatus::Skipped(_) => {
                    log::warn!("{:.<25}{:.>35}", name.yellow(), status.yellow())
                }
                SubscanModuleStatus::Failed(_) => {
                    log::error!("{:.<25}{:.>35}", name.red(), status.red())
                }
            }
        }
    }

    /// Start multiple [`SubscanModuleRunnerPool::runner`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::runner::SubscanModuleRunnerPool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let pool = SubscanModuleRunnerPool::new("foo.com".into());
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(1).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn spawn_runners(self: Arc<Self>, count: u64) {
        for _ in 0..count {
            self.runners.lock().await.spawn(self.clone().runner());
        }
    }

    /// Returns pool size, count of [`SubscanModuleRunnerPool::runner`] that spawned
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::runner::SubscanModuleRunnerPool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let pool = SubscanModuleRunnerPool::new("foo.com".into());
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(2).await;
    ///
    ///     assert_eq!(pool.len().await, 2);
    /// }
    /// ```
    pub async fn len(self: Arc<Self>) -> usize {
        self.runners.lock().await.len()
    }

    /// Returns [`true`] if any [`SubscanModuleRunnerPool::runner`] spawned otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::runner::SubscanModuleRunnerPool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let pool = SubscanModuleRunnerPool::new("foo.com".into());
    ///
    ///     assert!(pool.clone().is_empty().await);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(2).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn is_empty(self: Arc<Self>) -> bool {
        self.runners.lock().await.is_empty()
    }

    /// Submit [`SubscanModule`] into pool to be ran by ayn [`SubscanModuleRunnerPool::runner`] that
    /// have not any module its on
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::core::SubscanModule;
    /// use subscan::pools::runner::SubscanModuleRunnerPool;
    /// use subscan::modules::engines::google::Google;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let pool = SubscanModuleRunnerPool::new("foo.com".into());
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(1).await;
    ///     // submit module into pool
    ///     pool.clone().submit(module).await;
    ///     // join all runners in main thread
    ///     pool.clone().join().await;
    /// }
    /// ```
    pub async fn submit(self: Arc<Self>, module: SubscanModule) {
        self.input.tx.send(module).unwrap();
    }

    /// Join all [`SubscanModuleRunnerPool::runner`] in main thread
    ///
    /// # Panics
    ///
    /// If any error encountered while joining
    pub async fn join(self: Arc<Self>) {
        let mut runners = self.runners.lock().await;

        while let Some(result) = runners.join_next().await {
            if let Err(err) = result {
                panic!("Runner encountered an error: {:?}", err);
            }
        }
    }

    /// Get all subscan module results
    pub async fn results(&self) -> BTreeSet<SubscanModuleResult> {
        self.results.lock().await.clone()
    }
}
