use hickory_resolver::{
    config::{ResolverConfig as HickoryResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

use crate::types::{config::resolver::ResolverConfig, func::AsyncIPResolveFunc};

#[derive(Clone)]
pub struct Resolver {
    pub config: ResolverConfig,
    pub inner: TokioAsyncResolver,
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            config: ResolverConfig::default(),
            inner: TokioAsyncResolver::tokio(
                HickoryResolverConfig::default(),
                ResolverOpts::default(),
            ),
        }
    }
}

impl From<ResolverConfig> for Resolver {
    fn from(rconfig: ResolverConfig) -> Self {
        Self {
            config: rconfig.clone(),
            inner: TokioAsyncResolver::tokio(rconfig.config, rconfig.opts),
        }
    }
}

impl Resolver {
    pub async fn lookup_ip_future(&self) -> AsyncIPResolveFunc {
        self.config.lookup_ip_future().await
    }
}
