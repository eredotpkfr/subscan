/// In-memory cache module to store all modules
pub mod cache;
/// Includes CLI components
pub mod cli;
/// Project configuration utils
pub mod config;
/// Enumerations and project type definitions
pub mod enums;
/// Data extractors like
/// [`extractors::regex`], [`extractors::html`], etc.
pub mod extractors;
/// Trait implementations
pub mod interfaces;
/// All modules listed under this module, core components for subscan
pub mod modules;
/// HTTP requesters listed under this module
/// like [`requesters::chrome`], [`requesters::client`], etc.
pub mod requesters;
/// Project core type definitions
pub mod types;
/// Utilities for the handle different stuff things
pub mod utils;

use enums::{RequesterDispatcher, SubdomainExtractorDispatcher};
use interfaces::module::SubscanModuleInterface;
use std::collections::BTreeSet;
use tokio::sync::Mutex;
use types::core::APIKeyAsEnv;

/// Wrapper around a [`SubscanModuleInterface`] trait object
///
/// It has single field that stores [`SubscanModuleInterface`]
/// compatible object. Allows to access inner object's every
/// implemented method by using dynamic dispatching method
/// during run-time
///
/// Please follow up the [`struct@crate::cache::ALL_MODULES`]
/// to see pre-defined `subscan` modules
pub struct SubscanModule(Box<dyn SubscanModuleInterface>);

impl SubscanModule {
    /// Create new [`SubscanModule`] instance wrapped with a [`Mutex`]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use tokio::sync::Mutex;
    /// use subscan::SubscanModule;
    /// use subscan::interfaces::module::SubscanModuleInterface;
    /// use subscan::enums::{RequesterDispatcher, SubdomainExtractorDispatcher};
    /// use async_trait::async_trait;
    ///
    /// #[derive(Clone)]
    /// pub struct FooModule {}
    ///
    /// #[async_trait(?Send)]
    /// impl SubscanModuleInterface for FooModule {
    ///     async fn name(&self) -> &str {
    ///         &"foo-module"
    ///     }
    ///
    ///     async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
    ///         None
    ///     }
    ///
    ///     async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
    ///         None
    ///     }
    ///
    ///     async fn run(&mut self, domain: String) -> BTreeSet<String> {
    ///         // do something in `run` method
    ///         BTreeSet::new()
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let module = FooModule {};
    ///     let wrapped = SubscanModule::new(module.clone());
    ///
    ///     assert_eq!(wrapped.lock().await.name().await, module.name().await);
    ///
    ///     assert!(wrapped.lock().await.requester().await.is_none());
    ///     assert!(wrapped.lock().await.extractor().await.is_none());
    /// }
    /// ```
    pub fn new<M: 'static + SubscanModuleInterface>(module: M) -> Mutex<Self> {
        Mutex::new(Self(Box::new(module)))
    }

    pub async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        self.0.requester().await
    }

    pub async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        self.0.extractor().await
    }

    pub async fn run(&mut self, domain: String) -> BTreeSet<String> {
        self.0.run(domain).await
    }

    pub async fn name(&self) -> &str {
        self.0.name().await
    }

    pub async fn fetch_apikey(&self) -> APIKeyAsEnv {
        self.0.fetch_apikey().await
    }
}
