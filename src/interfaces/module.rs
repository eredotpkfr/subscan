use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use flume::Sender;
use tokio::sync::Mutex;

use super::requester::RequesterInterface;
use crate::{
    enums::{
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
        result::{OptionalSubscanModuleResult, SubscanModuleResult},
    },
    modules::{
        generics::{engine::GenericSearchEngineModule, integration::GenericIntegrationModule},
        integrations::{
            commoncrawl::CommonCrawl, dnsdumpstercrawler::DNSDumpsterCrawler, github::GitHub,
            netlas::Netlas, waybackarchive::WaybackArchive,
        },
        zonetransfer::ZoneTransfer,
    },
    types::{config::requester::RequesterConfig, env::SubscanModuleEnvs},
};

/// Generic `subscan` module trait definition to implement subdomain enumeration modules
///
/// Each module that will be implemented in the future must conform to this interface.
/// Summary it has single method that called `run` and it does whatever it has to do
#[async_trait]
#[enum_dispatch]
pub trait SubscanModuleInterface: Sync + Send {
    /// Returns module name, name should clarify what does module
    async fn name(&self) -> &str;
    /// Loads `.env` file and fetches module environment variables with variable name.
    /// If system environment variable set with same name, `.env` file will be overrode
    /// See the [`SubscanModuleEnvs`](crate::types::env::SubscanModuleEnvs) for details
    async fn envs(&self) -> SubscanModuleEnvs {
        self.name().await.into()
    }
    /// Returns module requester address as a mutable reference if available
    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>>;
    /// Returns module extractor reference if available
    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher>;
    /// Configure module requester instance
    async fn configure(&self, rconfig: RequesterConfig) {
        if let Some(requester) = self.requester().await {
            requester.lock().await.configure(rconfig).await;
        }
    }
    /// Just like a `main` method, when the module run this `run` method will be called.
    /// So this method should do everything
    async fn run(&mut self, domain: &str, results: Sender<OptionalSubscanModuleResult>);
}
