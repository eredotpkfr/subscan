use flume::{Receiver, Sender};
use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    interfaces::module::SubscanModuleInterface,
    types::core::{Subdomain, SubscanModule},
};
use std::{collections::BTreeSet, sync::Arc};

/// Module runner, basically listens given async channel and runs incoming `Subscan` modules
pub struct SubscanModuleRunner {
    receiver: Receiver<SubscanModule>,
}

/// Runner pool to run multiple  [`SubscanModuleRunner`]
pub struct SubscanModuleRunnerPool {
    results: Arc<Mutex<BTreeSet<Subdomain>>>,
    domain: String,
    runners: Mutex<JoinSet<()>>,
    input_tx: Sender<SubscanModule>,
    input_rx: Receiver<SubscanModule>,
}

impl SubscanModuleRunner {
    pub fn new(receiver: Receiver<SubscanModule>) -> Arc<Self> {
        Arc::new(Self { receiver })
    }

    /// Start listening on given async channel and handle incoming modules from channel
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pool::SubscanModuleRunner;
    /// use subscan::types::core::SubscanModule;
    /// use std::{sync::Arc, collections::BTreeSet};
    /// use tokio::sync::Mutex;
    /// use flume;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, rx) = flume::unbounded::<SubscanModule>();
    ///     let runner = SubscanModuleRunner::new(rx);
    ///     let results = Arc::new(Mutex::new(BTreeSet::new()));
    ///
    ///     // start to listen rx channel
    ///     runner.run("foo.com".into(), results).await
    /// }
    /// ```
    pub async fn run(self: Arc<Self>, domain: String, results: Arc<Mutex<BTreeSet<Subdomain>>>) {
        while let Ok(module) = self.receiver.try_recv() {
            let mut module = module.lock().await;

            log::info!("Running...{}", module.name().await);

            results.lock().await.extend(module.run(&domain).await);
        }
    }
}

impl SubscanModuleRunnerPool {
    pub fn new(domain: String) -> Arc<Self> {
        let runners = Mutex::new(JoinSet::new());
        let results = Arc::new(Mutex::new(BTreeSet::new()));

        let (input_tx, input_rx) = flume::unbounded::<SubscanModule>();

        Arc::new(Self {
            results,
            domain,
            runners,
            input_tx,
            input_rx,
        })
    }

    /// Start multiple [`SubscanModuleRunner`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pool::SubscanModuleRunnerPool;
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
            let mut runners = self.runners.lock().await;
            let runner = SubscanModuleRunner::new(self.input_rx.clone());

            runners.spawn(runner.run(self.domain.clone(), self.results.clone()));
        }
    }

    /// Returns pool size, count of [`SubscanModuleRunner`] that spawned
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pool::SubscanModuleRunnerPool;
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

    /// Returns [`true`] if any [`SubscanModuleRunner`] spawned otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pool::SubscanModuleRunnerPool;
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

    /// Submit [`SubscanModule`] into pool to be ran by ayn [`SubscanModuleRunner`] that
    /// have not any module its on
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::core::SubscanModule;
    /// use subscan::pool::SubscanModuleRunnerPool;
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
        self.input_tx.send(module).unwrap();
    }

    /// Join all [`SubscanModuleRunner`] in main thread
    pub async fn join(self: Arc<Self>) {
        let mut runners = self.runners.lock().await;

        while let Some(result) = runners.join_next().await {
            if let Err(err) = result {
                panic!("Runner encountered an error: {:?}", err);
            }
        }
    }

    /// Returns results
    pub async fn results(self: Arc<Self>) -> BTreeSet<Subdomain> {
        self.results.lock().await.clone()
    }
}
