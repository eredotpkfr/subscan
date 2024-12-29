use std::time::Duration;

use tokio::time::timeout;

use crate::{
    cli::commands::{
        brute::BruteCommandArgs, module::run::ModuleRunSubCommandArgs, scan::ScanCommandArgs,
    },
    constants::{DEFAULT_RESOLVER_CONCURRENCY, DEFAULT_RESOLVER_TIMEOUT},
    types::func::AsyncIPResolveFunc,
    utilities::net::lookup_host,
};

/// IP address resolver component configurations
#[derive(Clone, Debug)]
pub struct ResolverConfig {
    pub timeout: Duration,
    pub concurrency: u64,
    pub disabled: bool,
}

impl ResolverConfig {
    /// Returns future object that resolves IP address of any domain, if the
    /// [`disabled`](crate::types::config::resolver::ResolverConfig::disabled)
    /// option sets to [`true`] returns a future object that returns [`None`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use subscan::resolver::Resolver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut config = ResolverConfig::default();
    ///     let lookup_ip = config.lookup_host_future().await;
    ///
    ///     config.disabled = true;
    ///
    ///     let lookup_ip = config.lookup_host_future().await;
    ///     let resolver: Resolver = config.clone().into();
    ///
    ///     assert!(lookup_ip("foo.com".into()).await.is_none());
    /// }
    /// ```
    pub async fn lookup_host_future(&self) -> AsyncIPResolveFunc {
        let config = self.clone();

        if self.disabled {
            Box::new(|_: String| Box::pin(async move { None }))
        } else {
            Box::new(move |domain: String| {
                Box::pin(async move {
                    let future = timeout(config.timeout, lookup_host(&domain));

                    future.await.unwrap_or(None)
                })
            })
        }
    }
}

impl Default for ResolverConfig {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_RESOLVER_TIMEOUT,
            concurrency: DEFAULT_RESOLVER_CONCURRENCY,
            disabled: false,
        }
    }
}

impl From<ModuleRunSubCommandArgs> for ResolverConfig {
    /// Create [`ResolverConfig`] object from [`ModuleRunSubCommandArgs`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::cli::commands::module::run::ModuleRunSubCommandArgs;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// let args = ModuleRunSubCommandArgs {
    ///     resolver_concurrency: 100,
    ///     ..Default::default()
    /// };
    ///
    /// let config: ResolverConfig = args.clone().into();
    ///
    /// assert_eq!(config.concurrency, args.resolver_concurrency);
    /// ```
    fn from(args: ModuleRunSubCommandArgs) -> Self {
        Self {
            timeout: Duration::from_millis(args.resolver_timeout),
            concurrency: args.resolver_concurrency,
            disabled: args.resolver_disabled,
        }
    }
}

impl From<BruteCommandArgs> for ResolverConfig {
    /// Create [`ResolverConfig`] object from [`BruteCommandArgs`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::cli::commands::brute::BruteCommandArgs;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// let args = BruteCommandArgs {
    ///     resolver_concurrency: 100,
    ///     ..Default::default()
    /// };
    ///
    /// let config: ResolverConfig = args.clone().into();
    ///
    /// assert_eq!(config.concurrency, args.resolver_concurrency);
    /// ```
    fn from(args: BruteCommandArgs) -> Self {
        Self {
            timeout: Duration::from_millis(args.resolver_timeout),
            concurrency: args.resolver_concurrency,
            disabled: false,
        }
    }
}

impl From<ScanCommandArgs> for ResolverConfig {
    /// Create [`ResolverConfig`] object from [`ScanCommandArgs`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::cli::commands::scan::ScanCommandArgs;
    /// use subscan::types::config::resolver::ResolverConfig;
    ///
    /// let args = ScanCommandArgs {
    ///     resolver_concurrency: 100,
    ///     ..Default::default()
    /// };
    ///
    /// let config: ResolverConfig = args.clone().into();
    ///
    /// assert_eq!(config.concurrency, args.resolver_concurrency);
    /// ```
    fn from(args: ScanCommandArgs) -> Self {
        Self {
            timeout: Duration::from_millis(args.resolver_timeout),
            concurrency: args.resolver_concurrency,
            disabled: args.resolver_disabled,
        }
    }
}
