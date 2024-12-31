use async_trait::async_trait;

use crate::types::{config::resolver::ResolverConfig, func::AsyncIPResolveFunc};

/// IP lookup future trait implementation, lookup future is a future object that returns
/// IP address of given domain address so `lookup_host_future` method should return a future
/// object that resolvers IP address
#[async_trait]
pub trait LookUpHostFuture: Send + Sync {
    /// Should return `lookup_host` future object that acts according to `config.disabled`
    /// value. `lookup_host` future must resolve IP address by given host
    async fn lookup_host_future(&self) -> AsyncIPResolveFunc;
    /// Should return resolver configurations
    async fn config(&self) -> ResolverConfig;
}
