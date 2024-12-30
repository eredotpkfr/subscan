use async_trait::async_trait;

use crate::{
    interfaces::lookup::LookUpHostFuture,
    types::{config::resolver::ResolverConfig, func::AsyncIPResolveFunc},
};

/// IP address resolver component
#[derive(Clone, Default)]
pub struct Resolver {
    pub config: ResolverConfig,
}

impl From<ResolverConfig> for Resolver {
    fn from(rconfig: ResolverConfig) -> Self {
        Self {
            config: rconfig.clone(),
        }
    }
}

#[async_trait]
impl LookUpHostFuture for Resolver {
    /// Returns future object that resolves IP address of any domain, if the
    /// [`disabled`](crate::types::config::resolver::ResolverConfig::disabled)
    /// option sets to [`true`] returns a future object that returns [`None`]
    async fn lookup_host_future(&self) -> AsyncIPResolveFunc {
        self.config.lookup_host_future().await
    }

    /// Returns resolver config
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::resolver::Resolver;
    /// use subscan::types::config::resolver::ResolverConfig;
    /// use crate::subscan::interfaces::lookup::LookUpHostFuture;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let resolver = Resolver::from(ResolverConfig::default());
    ///
    ///     assert!(!resolver.config().await.disabled)
    /// }
    /// ```
    async fn config(&self) -> ResolverConfig {
        self.config.clone()
    }
}
