use hickory_resolver::{
    config::{ResolverConfig as HickoryResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use std::time::Duration;

use crate::{
    cli::commands::{
        brute::BruteCommandArgs, module::run::ModuleRunSubCommandArgs, scan::ScanCommandArgs,
    },
    constants::DEFAULT_RESOLVER_CONCURRENCY,
    types::func::AsyncIPResolveFunc,
};

#[derive(Clone, Debug)]
pub struct ResolverConfig {
    /// [`hickory_resolver::config::ResolverConfig`] instance
    pub config: HickoryResolverConfig,
    /// [`ResolverOpts`] instance
    pub opts: ResolverOpts,
    /// Thread counts of resolver instances
    pub concurrency: u64,
    /// Boolean flag to indicate IP resolver feature is disabled
    pub disabled: bool,
}

impl ResolverConfig {
    pub fn func(&self) -> AsyncIPResolveFunc {
        if self.disabled {
            Box::new(|_: &TokioAsyncResolver, _: String| Box::pin(async move { None }))
        } else {
            Box::new(|resolver: &TokioAsyncResolver, domain: String| {
                let resolver = resolver.clone();

                Box::pin(async move { resolver.lookup_ip(domain).await.ok()?.iter().next() })
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
