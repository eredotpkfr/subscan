use flume::{Receiver, Sender};
use hickory_resolver::TokioAsyncResolver;
use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    interfaces::module::SubscanModuleInterface,
    types::{
        config::resolver::ResolverConfig,
        core::{Subdomain, SubscanModule, UnboundedFlumeChannel},
        result::{item::SubscanModulePoolResultItem, pool::SubscanModulePoolResult},
    },
    utilities,
};
use std::sync::Arc;

/// Container for to store async channels in a single struct
struct SubscanModulePoolChannels {
    module: SubscanModuleChannel,
    subs: SubdomainChannel,
}

/// Container for result channel, stores receiver and sender instances
struct SubdomainChannel {
    tx: Sender<Subdomain>,
    rx: Receiver<Subdomain>,
}

/// Container for module channel, stores receiver and sender instances
struct SubscanModuleChannel {
    tx: Sender<SubscanModule>,
    rx: Receiver<SubscanModule>,
}

impl From<UnboundedFlumeChannel<SubscanModule>> for SubscanModuleChannel {
    fn from(channel: UnboundedFlumeChannel<SubscanModule>) -> Self {
        Self {
            tx: channel.0,
            rx: channel.1,
        }
    }
}

impl From<UnboundedFlumeChannel<Subdomain>> for SubdomainChannel {
    fn from(channel: UnboundedFlumeChannel<Subdomain>) -> Self {
        Self {
            tx: channel.0,
            rx: channel.1,
        }
    }
}

/// Subscan module pool to run modules and resolve IPs
pub struct SubscanModulePool {
    domain: String,
    rconfig: ResolverConfig,
    resolver: TokioAsyncResolver,
    result: Mutex<SubscanModulePoolResult>,
    channels: SubscanModulePoolChannels,
    workers: Mutex<JoinSet<()>>,
}

impl SubscanModulePool {
    pub fn new(domain: String, rconfig: ResolverConfig) -> Arc<Self> {
        let result = SubscanModulePoolResult::default().into();
        let channels = SubscanModulePoolChannels {
            module: flume::unbounded::<SubscanModule>().into(),
            subs: flume::unbounded::<Subdomain>().into(),
        };
        let resolver = TokioAsyncResolver::tokio(rconfig.config.clone(), rconfig.opts.clone());
        let workers = Mutex::new(JoinSet::new());

        Arc::new(Self {
            domain,
            rconfig,
            resolver,
            result,
            channels,
            workers,
        })
    }

    /// [`SubscanModule`] resolver method, simply resolves given subdomain's IP address
    async fn resolver(self: Arc<Self>) {
        let resolve_ip = self.rconfig.func();

        while let Ok(sub) = self.channels.subs.rx.try_recv() {
            let item = SubscanModulePoolResultItem {
                subdomain: sub.clone(),
                ip: resolve_ip(&self.resolver, sub).await,
            };

            self.result.lock().await.results.insert(item);
        }
    }

    /// [`SubscanModule`] runner method, simply calls `.run(` method
    async fn runner(self: Arc<Self>) {
        while let Ok(module) = self.channels.module.rx.try_recv() {
            let subresult = module.lock().await.run(&self.domain).await;
            let mut result = self.result.lock().await;

            result.statistics.module.push(subresult.stats());

            for sub in &subresult.subdomains {
                self.channels.subs.tx.send(sub.to_string()).unwrap();
            }

            utilities::log::result(subresult).await;
        }
    }

    /// Start multiple [`SubscanModulePool::runner`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let rconfig = ResolverConfig::default();
    ///     let pool = SubscanModulePool::new("foo.com".into(), rconfig);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(1).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn spawn_runners(self: Arc<Self>, count: u64) {
        for _ in 0..count {
            self.workers.lock().await.spawn(self.clone().runner());
        }
    }

    pub async fn spawn_resolvers(self: Arc<Self>, count: u64) {
        for _ in 0..count {
            self.workers.lock().await.spawn(self.clone().resolver());
        }
    }

    pub async fn start(self: Arc<Self>, concurrency: u64) {
        self.clone().spawn_runners(concurrency).await;
        self.clone().join().await;

        let time = self.result.lock().await.statistics.resolve.started();

        if !self.rconfig.disabled {
            log::info!(
                "IP resolution process started ({})",
                time.format("%H:%M:%S %Z")
            );
        }

        self.clone().spawn_resolvers(self.rconfig.concurrency).await;
        self.clone().join().await;

        self.result.lock().await.statistics.resolve.finished();
    }

    /// Returns pool size, count of [`SubscanModulePool::runner`] that spawned
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let rconfig = ResolverConfig::default();
    ///     let pool = SubscanModulePool::new("foo.com".into(), rconfig);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(2).await;
    ///
    ///     assert_eq!(pool.len().await, 2);
    /// }
    /// ```
    pub async fn len(self: Arc<Self>) -> usize {
        self.workers.lock().await.len()
    }

    /// Returns [`true`] if any [`SubscanModulePool::runner`] spawned otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let rconfig = ResolverConfig::default();
    ///     let pool = SubscanModulePool::new("foo.com".into(), rconfig);
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
        self.workers.lock().await.is_empty()
    }

    /// Submit [`SubscanModule`] into pool to be ran by ayn [`SubscanModulePool::runner`] that
    /// have not any module its on
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::core::SubscanModule;
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::modules::engines::google::Google;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let rconfig = ResolverConfig::default();
    ///     let pool = SubscanModulePool::new("foo.com".into(), rconfig);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(1).await;
    ///     // submit module into pool
    ///     pool.clone().submit(module).await;
    ///     // join all runners in main thread
    ///     pool.join().await;
    /// }
    /// ```
    pub async fn submit(self: Arc<Self>, module: SubscanModule) {
        self.channels.module.tx.send(module).unwrap();
    }

    /// Join all [`SubscanModulePool::runner`] in main thread
    ///
    /// # Panics
    ///
    /// If any error encountered while joining
    pub async fn join(self: Arc<Self>) {
        let mut runners = self.workers.lock().await;

        while let Some(result) = runners.join_next().await {
            if let Err(err) = result {
                panic!("Runner encountered an error: {:?}", err);
            }
        }
    }

    /// Get all subscan module results
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::core::SubscanModule;
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::modules::engines::google::Google;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let rconfig = ResolverConfig::default();
    ///     let pool = SubscanModulePool::new("foo.com".into(), rconfig);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(1).await;
    ///     // submit module into pool
    ///     pool.clone().submit(module).await;
    ///     // join all runners in main thread
    ///     pool.clone().join().await;
    ///
    ///     // do something with module result
    ///     let result = pool.result().await;
    /// }
    /// ```
    pub async fn result(&self) -> SubscanModulePoolResult {
        self.result.lock().await.clone()
    }
}
