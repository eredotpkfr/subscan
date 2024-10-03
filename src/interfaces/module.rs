use crate::{
    enums::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
    modules::generics::{
        api_integration::GenericAPIIntegrationModule, search_engine::GenericSearchEngineModule,
    },
    types::core::APIKeyAsEnv,
    utils::env,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::collections::BTreeSet;
use tokio::sync::Mutex;

/// Generic `subscan` module trait definition to implement
/// subdomain enumeration modules
///
/// Each module that will be implemented in the future
/// must conform to this interface. Summary it has
/// single method that called `run` and it does
/// whatever it has to do
///
/// # Examples
///
/// ```
/// use std::collections::BTreeSet;
/// use subscan::interfaces::module::SubscanModuleInterface;
/// use subscan::requesters::client::HTTPClient;
/// use subscan::extractors::regex::RegexExtractor;
/// use subscan::enums::{RequesterDispatcher, SubdomainExtractorDispatcher};
/// use async_trait::async_trait;
/// use tokio::sync::Mutex;
///
/// pub struct FooModule {
///     pub requester: Mutex<RequesterDispatcher>,
///     pub extractor: SubdomainExtractorDispatcher,
/// }
///
/// #[async_trait(?Send)]
/// impl SubscanModuleInterface for FooModule {
///     async fn name(&self) -> &str {
///         &"foo"
///     }
///
///     async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
///         Some(&self.requester)
///     }
///
///     async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
///         Some(&self.extractor)
///     }
///
///     async fn run(&mut self, domain: String) -> BTreeSet<String> {
///         BTreeSet::new()
///         // do something in `run` method
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
///     let extracator = RegexExtractor::default();
///
///     let mut foo = FooModule {
///         requester: Mutex::new(requester),
///         extractor: SubdomainExtractorDispatcher::RegexExtractor(extracator),
///     };
///
///     assert!(foo.requester().await.is_some());
///     assert!(foo.extractor().await.is_some());
///
///     assert_eq!(foo.name().await, "foo");
///
///     // do something with results
///     let results = foo.run("foo.com".to_string()).await;
/// }
/// ```
#[async_trait(?Send)]
#[enum_dispatch]
pub trait SubscanModuleInterface: Sync + Send {
    /// Returns module name, name should clarify what does module
    async fn name(&self) -> &str;
    /// Returns module requester address as a mutable reference
    /// if available
    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>>;
    /// Returns module extractor reference if available
    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher>;
    /// Just like a `main` method, when the module
    /// run this `run` method will be called, so this method
    /// should do everything
    async fn run(&mut self, domain: String) -> BTreeSet<String>;
    /// Loads `.env` file and fetches module API key with variable name. If system
    /// environment variable set with same name, `.env` file will be overrode
    /// See the [`get_subscan_module_apikey`](crate::utils::env::get_subscan_module_apikey)
    /// for details
    async fn fetch_apikey(&self) -> APIKeyAsEnv {
        env::get_subscan_module_apikey(self.name().await)
    }
}
