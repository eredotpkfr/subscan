use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
};

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    interfaces::lookup::LookUpHostFuture,
    resolver::Resolver,
    types::{
        config::subscan::SubscanConfig,
        core::{Subdomain, UnboundedFlumeChannel},
        result::{item::SubscanResultItem, pool::PoolResult},
    },
};

/// Subscan brute pool to make brute force attack asynchronously
pub struct SubscanBrutePool {
    concurrency: u64,
    result: Mutex<PoolResult>,
    resolver: Box<dyn LookUpHostFuture>,
    channel: UnboundedFlumeChannel<Option<Subdomain>>,
    workers: Mutex<JoinSet<()>>,
}

impl From<SubscanConfig> for Arc<SubscanBrutePool> {
    fn from(config: SubscanConfig) -> Self {
        Arc::new(SubscanBrutePool {
            concurrency: config.resolver.concurrency,
            result: PoolResult::default().into(),
            resolver: Resolver::boxed_from(config.resolver),
            channel: flume::unbounded::<Option<Subdomain>>().into(),
            workers: Mutex::new(JoinSet::new()),
        })
    }
}

impl SubscanBrutePool {
    pub fn new(concurrency: u64, resolver: Box<dyn LookUpHostFuture>) -> Arc<Self> {
        let result = PoolResult::default().into();
        let channel = flume::unbounded::<Option<Subdomain>>().into();
        let workers = Mutex::new(JoinSet::new());

        Arc::new(Self {
            concurrency,
            result,
            resolver,
            channel,
            workers,
        })
    }

    /// Bruter method, simply tries to resolve IP address of subdomain
    pub async fn bruter(self: Arc<Self>, domain: String) {
        let lookup_host = self.resolver.lookup_host_future().await;

        while let Ok(sub) = self.channel.rx.recv_async().await {
            if let Some(subdomain) = sub {
                let subdomain = format!("{subdomain}.{domain}");

                if let Some(ip) = lookup_host(subdomain.clone()).await {
                    let item = SubscanResultItem {
                        subdomain,
                        ip: Some(ip),
                    };

                    self.result.lock().await.items.insert(item);
                }
            } else {
                break;
            }
        }
    }

    /// Returns pool size, count of [`SubscanBrutePool::bruter`] that spawned
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanBrutePool::new(2, resolver);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters("foo.com").await;
    ///
    ///     assert_eq!(pool.clone().len().await, 2);
    ///
    ///     pool.clone().kill_bruters().await;
    ///     pool.clone().join().await;
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
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanBrutePool::new(2, resolver);
    ///
    ///     assert!(pool.clone().is_empty().await);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters("foo.com").await;
    ///
    ///     assert!(!pool.clone().is_empty().await);
    ///
    ///     pool.clone().kill_bruters().await;
    ///     pool.join().await;
    /// }
    /// ```
    pub async fn is_empty(self: Arc<Self>) -> bool {
        self.workers.lock().await.is_empty()
    }

    /// Start multiple [`SubscanBrutePool::bruter`] instance in a async pool
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanBrutePool::new(1, resolver);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters("foo.com").await;
    ///
    ///     assert!(!pool.clone().is_empty().await);
    ///
    ///     pool.clone().kill_bruters().await;
    ///     pool.join().await;
    /// }
    /// ```
    pub async fn spawn_bruters(self: Arc<Self>, domain: &str) {
        for _ in 0..self.concurrency {
            self.workers.lock().await.spawn(self.clone().bruter(domain.to_string()));
        }
    }

    /// Kill registered bruters
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanBrutePool::new(1, resolver);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters("foo.com").await;
    ///     pool.clone().kill_bruters().await;
    ///     pool.clone().join().await;
    ///
    ///     assert!(pool.is_empty().await);
    /// }
    /// ```
    pub async fn kill_bruters(self: Arc<Self>) {
        for _ in 0..self.concurrency {
            self.channel.tx.send(None).unwrap()
        }
    }

    /// Start brute force with wordlist file
    pub async fn start(self: Arc<Self>, domain: &str, wordlist: PathBuf) {
        let file = File::open(wordlist);
        let reader = BufReader::new(file.expect("Cannot read wordlist!"));

        self.clone().spawn_bruters(domain).await;

        for subdomain in reader.lines().map_while(Result::ok) {
            self.clone().submit(subdomain).await;
        }

        self.clone().kill_bruters().await;
        self.join().await;
    }

    /// Submit [`Subdomain`] into pool to be tried IP resolve
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::pools::brute::SubscanBrutePool;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanBrutePool::new(1, resolver);
    ///
    ///     // spawn bruters that listen async channel
    ///     pool.clone().spawn_bruters("foo.com").await;
    ///     // submit subdomain into pool
    ///     pool.clone().submit("api".into()).await;
    ///     // join all bruters in main thread
    ///     pool.clone().kill_bruters().await;
    ///     pool.join().await;
    /// }
    /// ```
    pub async fn submit(self: Arc<Self>, subdomain: Subdomain) {
        self.channel.tx.send(Some(subdomain)).unwrap();
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
                panic!("Runner encountered an error: {err:?}");
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
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::boxed_from(ResolverConfig::default());
    ///     let pool = SubscanBrutePool::new(1, resolver);
    ///
    ///     pool.clone().spawn_bruters("foo.com").await;
    ///     // submit subdomain into pool
    ///     pool.clone().submit("api".into()).await;
    ///     // join all bruters in main thread
    ///     pool.clone().kill_bruters().await;
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
