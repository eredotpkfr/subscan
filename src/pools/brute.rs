use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    resolver::Resolver,
    types::{
        config::subscan::SubscanConfig,
        core::{Subdomain, UnboundedFlumeChannel},
        result::{item::PoolResultItem, pool::PoolResult},
    },
};

/// Subscan brute pool to make brute force attack asynchronously
pub struct SubscanBrutePool {
    domain: String,
    result: Mutex<PoolResult>,
    resolver: Resolver,
    channel: UnboundedFlumeChannel<Subdomain>,
    workers: Mutex<JoinSet<()>>,
}

impl SubscanBrutePool {
    /// Create easily [`SubscanBrutePool`] from given domain and [`SubscanConfig`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::subscan::SubscanConfig;
    /// use subscan::pools::brute::SubscanBrutePool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = SubscanConfig::default();
    ///     let pool = SubscanBrutePool::from("foo.com", config);
    ///
    ///     assert_eq!(pool.len().await, 0);
    /// }
    /// ```
    pub fn from(domain: &str, config: SubscanConfig) -> Arc<Self> {
        Arc::new(Self {
            domain: domain.to_string(),
            result: PoolResult::default().into(),
            resolver: config.resolver.into(),
            channel: flume::unbounded::<Subdomain>().into(),
            workers: Mutex::new(JoinSet::new()),
        })
    }

    pub fn new(domain: String, resolver: Resolver) -> Arc<Self> {
        let result = PoolResult::default().into();
        let channel = flume::unbounded::<Subdomain>().into();
        let workers = Mutex::new(JoinSet::new());

        Arc::new(Self {
            domain,
            result,
            resolver,
            channel,
            workers,
        })
    }

    /// Bruter method, simply tries to resolve IP address of subdomain
    pub async fn bruter(self: Arc<Self>) {
        let lookup_ip = self.resolver.lookup_ip_future().await;

        while let Ok(sub) = self.channel.rx.try_recv() {
            let subdomain = format!("{sub}.{}", self.domain);

            if let Some(ip) = lookup_ip(&self.resolver, subdomain.clone()).await {
                let item = PoolResultItem {
                    subdomain,
                    ip: Some(ip),
                };

                self.result.lock().await.items.insert(item);
            }
        }
    }

    /// Start multiple [`SubscanBrutePool::bruter`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanBrutePool::new("foo.com".into(), resolver);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters(1).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn spawn_bruters(self: Arc<Self>, count: u64) {
        for _ in 0..count {
            self.workers.lock().await.spawn(self.clone().bruter());
        }
    }

    /// Returns pool size, count of [`SubscanBrutePool::bruter`] that spawned
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanBrutePool::new("foo.com".into(), resolver);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters(2).await;
    ///
    ///     assert_eq!(pool.len().await, 2);
    /// }
    /// ```
    pub async fn len(self: Arc<Self>) -> usize {
        self.workers.lock().await.len()
    }

    /// Returns [`true`] if any [`SubscanBrutePool::bruter`] spawned otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanBrutePool::new("foo.com".into(), resolver);
    ///
    ///     assert!(pool.clone().is_empty().await);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters(2).await;
    ///
    ///     assert!(!pool.is_empty().await);
    /// }
    /// ```
    pub async fn is_empty(self: Arc<Self>) -> bool {
        self.workers.lock().await.is_empty()
    }

    /// Submit [`Subdomain`] into pool to be tried IP resolve
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanBrutePool::new("foo.com".into(), resolver);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters(1).await;
    ///     // submit subdomain into pool
    ///     pool.clone().submit("api".into()).await;
    ///     // join all bruters in main thread
    ///     pool.join().await;
    /// }
    /// ```
    pub async fn submit(self: Arc<Self>, subdomain: Subdomain) {
        self.channel.tx.send(subdomain).unwrap();
    }

    /// Join all registered threads into main thread
    ///
    /// # Panics
    ///
    /// If any error encountered while joining
    pub async fn join(self: Arc<Self>) {
        let mut bruters = self.workers.lock().await;

        while let Some(result) = bruters.join_next().await {
            if let Err(err) = result {
                panic!("Runner encountered an error: {:?}", err);
            }
        }
    }

    /// Get pool result, includes statistics and items
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = ResolverConfig::default().into();
    ///     let pool = SubscanBrutePool::new("foo.com".into(), resolver);
    ///
    ///     // submit subdomain into pool
    ///     pool.clone().submit("api".into()).await;
    ///     // join all bruters in main thread
    ///     pool.clone().join().await;
    ///
    ///     // do something with pool result
    ///     let result = pool.result().await;
    /// }
    /// ```
    pub async fn result(&self) -> PoolResult {
        self.result.lock().await.clone()
    }
}
