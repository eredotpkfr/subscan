use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    enums::cache::CacheFilter,
    error::SubscanError::ModuleErrorWithResult,
    interfaces::{lookup::LookUpHostFuture, module::SubscanModuleInterface},
    resolver::Resolver,
    types::{
        config::subscan::SubscanConfig,
        core::{Subdomain, SubscanModule, UnboundedFlumeChannel},
        result::{item::PoolResultItem, pool::PoolResult, statistics::SubscanModuleStatistic},
    },
};

struct SubscanModulePoolChannels {
    module: UnboundedFlumeChannel<SubscanModule>,
    subs: UnboundedFlumeChannel<Option<Subdomain>>,
}

#[derive(Default)]
pub struct SubscanModulePoolWorkers {
    runners: Mutex<JoinSet<()>>,
    resolvers: Mutex<JoinSet<()>>,
}

impl SubscanModulePoolWorkers {
    /// Returns total registered workers count
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePoolWorkers;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let workers = SubscanModulePoolWorkers::default();
    ///
    ///     assert_eq!(workers.len().await, 0);
    /// }
    /// ```
    pub async fn len(&self) -> usize {
        self.runners.lock().await.len() + self.resolvers.lock().await.len()
    }

    /// Returns `true` any worker thread is alive otherwise `false`
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePoolWorkers;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let workers = SubscanModulePoolWorkers::default();
    ///
    ///     assert!(workers.is_empty().await);
    /// }
    /// ```
    pub async fn is_empty(&self) -> bool {
        self.runners.lock().await.is_empty() && self.resolvers.lock().await.is_empty()
    }
}

/// Subscan module pool to run modules and resolve IPs
pub struct SubscanModulePool {
    domain: String,
    result: Mutex<PoolResult>,
    resolver: Box<dyn LookUpHostFuture>,
    filter: CacheFilter,
    channels: SubscanModulePoolChannels,
    workers: SubscanModulePoolWorkers,
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
            result: PoolResult::default().into(),
            resolver: Box::new(Resolver::from(config.resolver)),
            channels: SubscanModulePoolChannels {
                module: flume::unbounded::<SubscanModule>().into(),
                subs: flume::unbounded::<Option<Subdomain>>().into(),
            },
            workers: SubscanModulePoolWorkers::default(),
            filter: config.filter,
        })
    }

    pub fn new(
        domain: String,
        resolver: Box<dyn LookUpHostFuture>,
        filter: CacheFilter,
    ) -> Arc<Self> {
        let result = PoolResult::default().into();
        let channels = SubscanModulePoolChannels {
            module: flume::unbounded::<SubscanModule>().into(),
            subs: flume::unbounded::<Option<Subdomain>>().into(),
        };
        let workers = SubscanModulePoolWorkers::default();

        Arc::new(Self {
            domain,
            result,
            resolver,
            filter,
            channels,
            workers,
        })
    }

    /// Returns pool size, count of [`SubscanModulePool::runner`] that spawned
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver: Resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), Box::new(resolver), NoFilter);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(2).await;
    ///
    ///     assert_eq!(pool.len().await, 2);
    /// }
    /// ```
    pub async fn len(self: Arc<Self>) -> usize {
        self.workers.len().await
    }

    /// Returns [`true`] if any [`SubscanModulePool::runner`] spawned otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver: Resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), Box::new(resolver), NoFilter);
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
        self.workers.is_empty().await
    }

    /// [`SubscanModule`] resolver method, simply resolves given subdomain's IP address
    pub async fn resolver(self: Arc<Self>) {
        let lookup_host = self.resolver.lookup_host_future().await;

        while let Ok(sub) = self.channels.subs.rx.recv_async().await {
            if let Some(subdomain) = sub {
                let item = PoolResultItem {
                    subdomain: subdomain.clone(),
                    ip: lookup_host(subdomain).await,
                };

                self.result.lock().await.items.insert(item);
            } else {
                break;
            }
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

                    for sub in subresult.valids(&self.domain) {
                        self.channels.subs.tx.send(Some(sub.to_string())).unwrap()
                    }
                } else {
                    let error = subresult.unwrap_err();
                    let stats = error.stats(name);

                    stats.status.log(name);
                    self.result.lock().await.statistic(stats).await;
                }
            } else {
                let name = module.name().await;
                let stats = SubscanModuleStatistic::skipped(name);

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
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver: Resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), Box::new(resolver), NoFilter);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners(1).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn spawn_runners(self: Arc<Self>, count: u64) {
        for _ in 0..count {
            self.workers
                .runners
                .lock()
                .await
                .spawn(self.clone().runner());
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
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver: Resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), Box::new(resolver), NoFilter);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_resolvers(1).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn spawn_resolvers(self: Arc<Self>, count: u64) {
        for _ in 0..count {
            self.workers
                .resolvers
                .lock()
                .await
                .spawn(self.clone().resolver());
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
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver: Resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), Box::new(resolver), NoFilter);
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
        let rconfig = self.resolver.config().await;

        self.clone().spawn_resolvers(rconfig.concurrency).await;
        self.clone().spawn_runners(concurrency).await;
        self.clone().join_runners().await;

        for _ in 0..rconfig.concurrency {
            self.channels.subs.tx.send(None).unwrap();
        }

        self.clone().join_resolvers().await;
        self.result.lock().await.statistics.resolve.finished();
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
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver: Resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), Box::new(resolver), NoFilter);
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

    /// Join registered runner threads into main thread
    ///
    /// # Panics
    ///
    /// If any error encountered while joining
    pub async fn join_runners(self: Arc<Self>) {
        let mut runners = self.workers.runners.lock().await;

        while let Some(result) = runners.join_next().await {
            if let Err(err) = result {
                panic!("Runner encountered an error: {:?}", err);
            }
        }
    }

    /// Join registered resolver threads into main thread
    ///
    /// # Panics
    ///
    /// If any error encountered while joining
    pub async fn join_resolvers(self: Arc<Self>) {
        let mut resolvers = self.workers.resolvers.lock().await;

        while let Some(result) = resolvers.join_next().await {
            if let Err(err) = result {
                panic!("Runner encountered an error: {:?}", err);
            }
        }
    }

    /// Join all registered threads into main thread
    ///
    /// # Panics
    ///
    /// If any error encountered while joining
    pub async fn join(self: Arc<Self>) {
        self.clone().join_resolvers().await;
        self.join_runners().await;
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
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver: Resolver = ResolverConfig::default().into();
    ///     let pool = SubscanModulePool::new("foo.com".into(), Box::new(resolver), NoFilter);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // submit module into pool
    ///     pool.clone().submit(module).await;
    ///     // start pool with a concurrency level and join all
    ///     // runners in main thread
    ///     pool.clone().start(1).await;
    ///
    ///     // do something with pool result
    ///     let result = pool.result().await;
    /// }
    /// ```
    pub async fn result(&self) -> PoolResult {
        self.result.lock().await.clone()
    }
}
