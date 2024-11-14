use hickory_resolver::config::{ResolverConfig as HickoryResolverConfig, ResolverOpts};
use std::time::Duration;

use crate::{
    cli::commands::{
        brute::BruteCommandArgs, module::run::ModuleRunSubCommandArgs, scan::ScanCommandArgs,
    },
    constants::DEFAULT_RESOLVER_CONCURRENCY,
    resolver::Resolver,
    types::func::AsyncIPResolveFunc,
};

/// IP address resolver component configurations
#[derive(Clone, Debug)]
pub struct ResolverConfig {
    /// Inner [`ResolverConfig`](hickory_resolver::config::ResolverConfig) instance
    pub config: HickoryResolverConfig,
    /// Inner [`ResolverOpts`] instance
    pub opts: ResolverOpts,
    /// Thread counts of resolver instances
    pub concurrency: u64,
    /// Boolean flag to indicate IP resolver feature is disabled
    pub disabled: bool,
}

impl ResolverConfig {
    /// Returns future object that resolves IP address of any domain, if the
    /// [`disabled`](crate::types::config::resolver::ResolverConfig::disabled)
    /// option sets to [`true`] returns a future object that returns [`None`]
    pub async fn lookup_ip_future(&self) -> AsyncIPResolveFunc {
        if self.disabled {
            Box::new(|_: &Resolver, _: String| Box::pin(async move { None }))
        } else {
            Box::new(|resolver: &Resolver, domain: String| {
                let resolver = resolver.clone();

                Box::pin(async move { resolver.inner.lookup_ip(domain).await.ok()?.iter().next() })
            })
        }
    }
}

impl Default for ResolverConfig {
    fn default() -> Self {
        Self {
            config: HickoryResolverConfig::default(),
            opts: ResolverOpts::default(),
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
        let mut options = ResolverOpts::default();

        options.timeout = Duration::from_secs(args.resolver_timeout);

        Self {
            config: HickoryResolverConfig::default(),
            opts: options,
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
        let mut options = ResolverOpts::default();

        options.timeout = Duration::from_secs(args.resolver_timeout);

        Self {
            config: HickoryResolverConfig::default(),
            opts: options,
            concurrency: args.resolver_concurrency,
            disabled: args.resolver_disabled,
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
        let mut options = ResolverOpts::default();

        options.timeout = Duration::from_secs(args.resolver_timeout);

        Self {
            config: HickoryResolverConfig::default(),
            opts: options,
            concurrency: args.resolver_concurrency,
            disabled: args.resolver_disabled,
        }
    }
}
