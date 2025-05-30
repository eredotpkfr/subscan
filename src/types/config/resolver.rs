use std::time::Duration;

use hickory_resolver::config::ResolverConfig as HickoryResolverConfig;

use crate::{
    cli::commands::{
        brute::BruteCommandArgs, module::run::ModuleRunSubCommandArgs, scan::ScanCommandArgs,
    },
    constants::{DEFAULT_RESOLVER_CONCURRENCY, DEFAULT_RESOLVER_TIMEOUT},
    utilities::net,
};

/// IP address resolver component configurations
#[derive(Clone, Debug)]
pub struct ResolverConfig {
    pub inner: HickoryResolverConfig,
    pub concurrency: u64,
    pub disabled: bool,
    pub timeout: Duration,
}

impl Default for ResolverConfig {
    fn default() -> Self {
        Self {
            inner: net::read_system_ns_conf().unwrap_or(HickoryResolverConfig::cloudflare()),
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
            inner: args.resolver_list.map_or(
                net::read_system_ns_conf().unwrap_or(HickoryResolverConfig::cloudflare()),
                net::read_resolver_list_file,
            ),
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
            inner: args.resolver_list.map_or(
                net::read_system_ns_conf().unwrap_or(HickoryResolverConfig::cloudflare()),
                net::read_resolver_list_file,
            ),
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
            inner: args.resolver_list.map_or(
                net::read_system_ns_conf().unwrap_or(HickoryResolverConfig::cloudflare()),
                net::read_resolver_list_file,
            ),
            timeout: Duration::from_millis(args.resolver_timeout),
            concurrency: args.resolver_concurrency,
            disabled: args.resolver_disabled,
        }
    }
}
