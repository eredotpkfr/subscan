use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    constants::LOG_TIME_FORMAT,
    enums::cache::CacheFilter,
    error::SubscanError::ModuleErrorWithResult,
    interfaces::module::SubscanModuleInterface,
    resolver::Resolver,
    types::{
        config::subscan::SubscanConfig,
        core::{Subdomain, SubscanModule, UnboundedFlumeChannel, UnboundedFlumeChannelTuple},
        result::{
            item::SubscanModulePoolResultItem, pool::SubscanModulePoolResult,
            statistics::SubscanModuleStatistics,
        },
    },
};

struct SubscanModulePoolChannels {
    module: UnboundedFlumeChannel<SubscanModule>,
    subs: UnboundedFlumeChannel<Subdomain>,
}

impl From<UnboundedFlumeChannelTuple<SubscanModule>> for UnboundedFlumeChannel<SubscanModule> {
    fn from(channel: UnboundedFlumeChannelTuple<SubscanModule>) -> Self {
        Self {
            tx: channel.0,
            rx: channel.1,
        }
    }
}

impl From<UnboundedFlumeChannelTuple<Subdomain>> for UnboundedFlumeChannel<Subdomain> {
    fn from(channel: UnboundedFlumeChannelTuple<Subdomain>) -> Self {
        Self {
            tx: channel.0,
            rx: channel.1,
        }
    }
}

/// Subscan module pool to run modules and resolve IPs
pub struct SubscanModulePool {
    domain: String,
    result: Mutex<SubscanModulePoolResult>,
    resolver: Resolver,
    filter: CacheFilter,
    channels: SubscanModulePoolChannels,
    workers: Mutex<JoinSet<()>>,
}

impl SubscanModulePool {
    /// Create easily [`SubscanModulePool`] from given domain and [`SubscanConfig`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::subscan::SubscanConfig;
    /// use subscan::pools::module::SubscanModulePool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = SubscanConfig::default();
    ///     let pool = SubscanModulePool::from("foo.com", config);
    ///
    ///     assert_eq!(pool.len().await, 0);
    /// }
    /// ```
    pub fn from(domain: &str, config: SubscanConfig) -> Arc<Self> {
        Arc::new(Self {
            domain: domain.to_string(),
            result: SubscanModulePoolResult::default().into(),
            resolver: config.resolver.into(),
            channels: SubscanModulePoolChannels {
                module: flume::unbounded::<SubscanModule>().into(),
                subs: flume::unbounded::<Subdomain>().into(),
            },
            workers: Mutex::new(JoinSet::new()),
            filter: config.filter,
        })
    }

    pub fn new(domain: String, resolver: Resolver, filter: CacheFilter) -> Arc<Self> {
        let result = SubscanModulePoolResult::default().into();
        let channels = SubscanModulePoolChannels {
            module: flume::unbounded::<SubscanModule>().into(),
            subs: flume::unbounded::<Subdomain>().into(),
        };
        let workers = Mutex::new(JoinSet::new());

        Arc::new(Self {
            domain,
            result,
            resolver,
            filter,
            channels,
            workers,
        })
    }

    /// [`SubscanModule`] resolver method, simply resolves given subdomain's IP address
    pub async fn resolver(self: Arc<Self>) {
        let lookup_ip = self.resolver.lookup_ip_future().await;

        while let Ok(sub) = self.channels.subs.rx.try_recv() {
            let item = SubscanModulePoolResultItem {
                subdomain: sub.clone(),
                ip: lookup_ip(&self.resolver, sub).await,
            };

            self.result.lock().await.items.insert(item);
        }
    }

    /// [`SubscanModule`] runner method, simply calls `.run(` method
    pub async fn runner(self: Arc<Self>) {
        while let Ok(module) = self.channels.module.rx.try_recv() {
            let mut module = module.lock().await;

            if !self.filter.is_filtered(module.name().await).await {
                let subresult = module.run(&self.domain).await;
                let name = module.name().await;

                if let Ok(subresult) | Err(ModuleErrorWithResult(subresult)) = subresult {
                    let stats = subresult.stats().await;

                    stats.status.log(name);
                    self.result.lock().await.statistic(stats).await;

                    for sub in &subresult.subdomains {
                        self.channels.subs.tx.send(sub.to_string()).unwrap()
                    }
                } else {
                    let error = subresult.unwrap_err();
                    let stats = error.stats(name);

                    stats.status.log(name);
                    self.result.lock().await.statistic(stats).await;
                }
            } else {
                let name = module.name().await;
                let stats = SubscanModuleStatistics::skipped(name);

                stats.status.log(name);
                self.result.lock().await.statistic(stats).await;
            }
        }
    }

    /// Start multiple [`SubscanModulePool::runner`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), resolver, NoFilter);
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

    /// Start multiple [`SubscanModulePool::resolver`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), resolver, NoFilter);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_resolvers(1).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn spawn_resolvers(self: Arc<Self>, count: u64) {
        for _ in 0..count {
            self.workers.lock().await.spawn(self.clone().resolver());
        }
    }

    /// Start pool execution, runs all submitted modules and resolves IP
    /// addresses of discovered subdomains
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::core::SubscanModule;
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::modules::engines::google::Google;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), resolver, NoFilter);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // submit module into pool
    ///     pool.clone().submit(module).await;
    ///     // start pool with a concurrency level and join all
    ///     // runners in main thread
    ///     pool.start(1).await;
    /// }
    /// ```
    pub async fn start(self: Arc<Self>, concurrency: u64) {
        self.clone().spawn_runners(concurrency).await;
        self.clone().join().await;

        let time = self.result.lock().await.statistics.resolve.started();
        let rconcurrency = self.resolver.config.concurrency;

        if !self.resolver.config.disabled {
            log::info!(
                "IP resolution process started ({})",
                time.format(LOG_TIME_FORMAT)
            );
        }

        self.clone().spawn_resolvers(rconcurrency).await;
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
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), resolver, NoFilter);
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
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), resolver, NoFilter);
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
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), resolver, NoFilter);
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

    /// Join all registered threads into main thread
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

    /// Get pool result, includes module results as a subresult
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::core::SubscanModule;
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::modules::engines::google::Google;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), resolver, NoFilter);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // submit module into pool
    ///     pool.clone().submit(module).await;
    ///     // start pool with a concurrency level and join all
    ///     // runners in main thread
    ///     pool.clone().start(1).await;
    ///
    ///     // do something with module result
    ///     let result = pool.result().await;
    /// }
    /// ```
    pub async fn result(&self) -> SubscanModulePoolResult {
        self.result.lock().await.clone()
    }
}
