use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use async_trait::async_trait;
use subscan::{
    interfaces::lookup::LookUpHostFuture,
    types::{config::resolver::ResolverConfig, func::AsyncIPResolveFunc},
};

use crate::common::constants::LOCAL_HOST;

#[derive(Default)]
pub struct MockResolver {
    config: ResolverConfig,
}

impl MockResolver {
    pub fn new(config: ResolverConfig) -> Self {
        Self { config }
    }

    pub fn boxed(config: ResolverConfig) -> Box<Self> {
        Box::new(Self::new(config))
    }

    pub fn default_boxed() -> Box<Self> {
        Box::new(Self::default())
    }
}

impl From<ResolverConfig> for MockResolver {
    fn from(config: ResolverConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl LookUpHostFuture for MockResolver {
    async fn lookup_host_future(&self) -> AsyncIPResolveFunc {
        if self.config.disabled {
            Box::new(|_: String| Box::pin(async move { None }))
        } else {
            Box::new(move |_: String| {
                Box::pin(async move { Some(IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap())) })
            })
        }
    }

    async fn config(&self) -> ResolverConfig {
        self.config.clone()
    }
}
