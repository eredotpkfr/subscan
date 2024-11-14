use crate::types::{config::resolver::ResolverConfig, func::AsyncIPResolveFunc};
use hickory_resolver::TokioAsyncResolver;

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
    pub async fn lookup_ip_future(&self) -> AsyncIPResolveFunc {
        self.config.lookup_ip_future().await
    }
}
