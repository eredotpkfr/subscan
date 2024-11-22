use hickory_resolver::TokioAsyncResolver;

use crate::types::{config::resolver::ResolverConfig, func::AsyncIPResolveFunc};

/// IP address resolver component
#[derive(Clone)]
pub struct Resolver {
    pub config: ResolverConfig,
    pub inner: TokioAsyncResolver,
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
    /// Returns future object that resolves IP address of any domain, if the
    /// [`disabled`](crate::types::config::resolver::ResolverConfig::disabled)
    /// option sets to [`true`] returns a future object that returns [`None`]
    pub async fn lookup_ip_future(&self) -> AsyncIPResolveFunc {
        self.config.lookup_ip_future().await
    }
}
