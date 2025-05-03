use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    enums::result::{OptionalSubscanModuleResult, SubscanModuleResult},
    interfaces::{lookup::LookUpHostFuture, module::SubscanModuleInterface},
    resolver::Resolver,
    types::{
        config::{pool::PoolConfig, subscan::SubscanConfig},
        core::{SubscanModule, UnboundedFlumeChannel},
        result::{pool::PoolResult, statistics::SubscanModuleStatistic},
    },
};

struct SubscanModulePoolChannels {
    module: UnboundedFlumeChannel<Option<SubscanModule>>,
    results: UnboundedFlumeChannel<OptionalSubscanModuleResult>,
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
    config: PoolConfig,
    resolver: Box<dyn LookUpHostFuture>,
    result: Mutex<PoolResult>,
    channels: SubscanModulePoolChannels,
    workers: SubscanModulePoolWorkers,
}

impl From<SubscanConfig> for Arc<SubscanModulePool> {
    fn from(config: SubscanConfig) -> Self {
        Arc::new(SubscanModulePool {
            config: config.clone().into(),
            resolver: Resolver::boxed_from(config.resolver),
            result: PoolResult::default().into(),
            channels: SubscanModulePoolChannels {
                module: flume::unbounded::<Option<SubscanModule>>().into(),
                results: flume::unbounded::<OptionalSubscanModuleResult>().into(),
            },
            workers: SubscanModulePoolWorkers::default(),
        })
    }
}

impl SubscanModulePool {
    pub fn new(config: PoolConfig, resolver: Box<dyn LookUpHostFuture>) -> Arc<Self> {
        let result = PoolResult::default().into();
        let channels = SubscanModulePoolChannels {
            module: flume::unbounded::<Option<SubscanModule>>().into(),
            results: flume::unbounded::<OptionalSubscanModuleResult>().into(),
        };
        let workers = SubscanModulePoolWorkers::default();

        Arc::new(Self {
            config,
            resolver,
            result,
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
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 2,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners("foo.com").await;
    ///
    ///     assert_eq!(pool.clone().len().await, 2);
    ///
    ///     pool.clone().kill_runners().await;
    ///     pool.join_runners().await;
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
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///
    ///     assert!(pool.clone().is_empty().await);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners("foo.com").await;
    ///
    ///     assert!(!pool.clone().is_empty().await);
    ///
    ///     pool.clone().kill_runners().await;
    ///     pool.join_runners().await;
    /// }
    /// ```
    pub async fn is_empty(self: Arc<Self>) -> bool {
        self.workers.is_empty().await
    }

    /// [`SubscanModule`] resolver method, simply resolves given subdomain's IP address
    pub async fn resolver(self: Arc<Self>) {
        let _lookup_host = self.resolver.lookup_host_future().await;

        while let Ok(msg) = self.channels.results.rx.recv_async().await {
            if let Some(result) = msg.as_ref() {
                match result {
                    SubscanModuleResult::SubscanModuleResultItem(item) => {
                        println!("{:#?}", item)
                    }
                    SubscanModuleResult::SubscanModuleStatus(status) => {
                        println!("{:#?}", status);
                    }
                }
            } else {
                break;
            }
        }
    }

    /// [`SubscanModule`] runner method, simply calls `.run(` method
    pub async fn runner(self: Arc<Self>, domain: String) {
        while let Ok(msg) = self.channels.module.rx.recv_async().await {
            if let Some(module) = msg {
                let mut module = module.lock().await;

                if self.config.filter.is_filtered(module.name().await).await {
                    let name = module.name().await.to_string();
                    let stats = SubscanModuleStatistic::skipped();

                    stats.status.log(&name);
                    self.result.lock().await.statistics.insert(name, stats);
                } else {
                    module.run(&domain, self.channels.results.tx.clone()).await;
                }
            } else {
                break;
            }
        }
    }

    /// Start multiple [`SubscanModulePool::runner`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners("foo.com").await;
    ///
    ///     assert!(!pool.clone().is_empty().await);
    ///
    ///     pool.clone().kill_runners().await;
    ///     pool.join_runners().await;
    /// }
    /// ```
    pub async fn spawn_runners(self: Arc<Self>, domain: &str) {
        for _ in 0..self.config.concurrency {
            self.workers
                .runners
                .lock()
                .await
                .spawn(self.clone().runner(domain.to_string()));
        }
    }

    /// Kill all [`SubscanModulePool::runner`] instances in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners("foo.com").await;
    ///     pool.clone().kill_runners().await;
    ///     pool.clone().join_runners().await;
    ///
    ///     assert!(pool.is_empty().await);
    /// }
    /// ```
    pub async fn kill_runners(self: Arc<Self>) {
        for _ in 0..self.config.concurrency {
            self.channels.module.tx.send(None).unwrap()
        }
    }

    /// Start multiple [`SubscanModulePool::resolver`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_resolvers().await;
    ///
    ///     assert!(!pool.clone().is_empty().await);
    ///
    ///     pool.clone().kill_resolvers().await;
    ///     pool.join_resolvers().await;
    /// }
    /// ```
    pub async fn spawn_resolvers(self: Arc<Self>) {
        for _ in 0..self.resolver.config().await.concurrency {
            self.workers
                .resolvers
                .lock()
                .await
                .spawn(self.clone().resolver());
        }
    }

    /// Kill all [`SubscanModulePool::resolver`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::module::SubscanModulePool;
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_resolvers().await;
    ///     pool.clone().kill_resolvers().await;
    ///     pool.clone().join_resolvers().await;
    ///
    ///     assert!(pool.is_empty().await);
    /// }
    /// ```
    pub async fn kill_resolvers(self: Arc<Self>) {
        for _ in 0..self.resolver.config().await.concurrency {
            self.channels
                .results
                .tx
                .send(OptionalSubscanModuleResult(None))
                .unwrap()
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
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     pool.start("foo.com", &vec![module]).await;
    /// }
    /// ```
    pub async fn start(self: Arc<Self>, domain: &str, modules: &Vec<SubscanModule>) {
        self.clone().spawn_resolvers().await;
        self.clone().spawn_runners(domain).await;

        for module in modules {
            self.clone().submit(module.clone()).await;
        }

        self.clone().kill_runners().await;
        self.clone().join_runners().await;

        self.clone().kill_resolvers().await;
        self.clone().join_resolvers().await;
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
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // spawn resolvers
    ///     pool.clone().spawn_resolvers().await;
    ///     // spawn runners that listen async channel
    ///     pool.clone().spawn_runners("foo.com").await;
    ///     // submit module into pool
    ///     pool.clone().submit(module).await;
    ///
    ///     // kill runners and join into main thread
    ///     pool.clone().kill_runners().await;
    ///     pool.clone().join_runners().await;
    ///
    ///     // kill resolvers and join into main thread
    ///     pool.clone().kill_resolvers().await;
    ///     pool.clone().join_resolvers().await;
    /// }
    /// ```
    pub async fn submit(self: Arc<Self>, module: SubscanModule) {
        self.channels.module.tx.send(Some(module)).unwrap();
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
                panic!("Runner encountered an error: {err:?}");
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
                panic!("Runner encountered an error: {err:?}");
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
    /// use subscan::types::config::{resolver::ResolverConfig, pool::PoolConfig};
    /// use subscan::enums::cache::CacheFilter::NoFilter;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let config = PoolConfig {
    ///         filter: NoFilter,
    ///         concurrency: 1,
    ///         print: false,
    ///         output: None
    ///     };
    ///
    ///     let pool = SubscanModulePool::new(config, resolver);
    ///     let module = SubscanModule::from(Google::dispatcher());
    ///
    ///     // start pool
    ///     pool.clone().start("foo.com", &vec![module]).await;
    ///
    ///     // do something with pool result
    ///     let result = pool.result().await;
    /// }
    /// ```
    pub async fn result(&self) -> PoolResult {
        self.result.lock().await.clone()
    }
}
