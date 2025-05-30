use async_trait::async_trait;
use hickory_client::proto::runtime::TokioRuntimeProvider;
use hickory_resolver::{
    name_server::{GenericConnector, TokioConnectionProvider},
    Resolver as HickoryResolver,
};
use tokio::time;

use crate::{
    interfaces::lookup::LookUpHostFuture,
    types::{config::resolver::ResolverConfig, func::AsyncIPResolveFunc},
};

/// IP address resolver component
#[derive(Clone)]
pub struct Resolver {
    pub config: ResolverConfig,
    pub inner: HickoryResolver<GenericConnector<TokioRuntimeProvider>>,
}

impl Default for Resolver {
    fn default() -> Self {
        let config = ResolverConfig::default();
        let provider = TokioConnectionProvider::default();
        let inner = HickoryResolver::builder_with_config(config.clone().inner, provider).build();

        Self { inner, config }
    }
}

impl From<ResolverConfig> for Resolver {
    fn from(config: ResolverConfig) -> Self {
        let provider = TokioConnectionProvider::default();
        let inner = HickoryResolver::builder_with_config(config.clone().inner, provider).build();

        Self { inner, config }
    }
}

impl Resolver {
    pub fn boxed_from(config: ResolverConfig) -> Box<Self> {
        let provider = TokioConnectionProvider::default();
        let inner = HickoryResolver::builder_with_config(config.clone().inner, provider).build();

        Box::new(Self { inner, config })
    }
}

#[async_trait]
impl LookUpHostFuture for Resolver {
    /// Returns future object that resolves IP address of any domain, if the
    /// [`disabled`](crate::types::config::resolver::ResolverConfig::disabled)
    /// option sets to [`true`] returns a future object that returns [`None`]
    async fn lookup_host_future(&self) -> AsyncIPResolveFunc {
        if !self.config.disabled {
            let timeout = self.config.timeout;
            let inner = self.inner.clone();

            Box::new(move |domain: String| {
                let resolver = inner.clone();

                Box::pin({
                    async move {
                        time::timeout(timeout, resolver.lookup_ip(&domain))
                            .await
                            .ok()?
                            .ok()?
                            .iter()
                            .next()
                    }
                })
            })
        } else {
            Box::new(|_: String| Box::pin(async move { None }))
        }
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
