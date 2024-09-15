/// In-memory cache module to store requesters and modules
pub mod cache;
/// Includes CLI components
pub mod cli;
/// Enumerations and project type definitions
pub mod enums;
/// Data extractors like
/// [`extractors::regex`], [`extractors::html`], etc.
pub mod extractors;
/// Thirty party integration modules
pub mod integrations;
/// Trait implementations
pub mod interfaces;
/// All modules listed under this module, core components for subscan
pub mod modules;
/// HTTP requesters listed under this module
/// like [`requesters::chrome`], [`requesters::client`], etc.
pub mod requesters;
/// Porject core type definitions
pub mod types;
/// Utilities for the handle different stuff things
pub mod utils;

use interfaces::module::SubscanModuleInterface;
use tokio::sync::Mutex;

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
    /// use subscan::SubscanModule;
    /// use subscan::interfaces::module::SubscanModuleInterface;
    /// use async_trait::async_trait;
    ///
    /// #[derive(Clone)]
    /// pub struct FooModule {}
    ///
    /// #[async_trait(?Send)]
    /// impl SubscanModuleInterface for FooModule {
    ///     async fn name(&self) -> String {
    ///         String::from("Foo")
    ///     }
    ///     async fn run(&mut self, domain: String) {
    ///         // do something in `run` method
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let module = FooModule {};
    ///     let wrapped = SubscanModule::new(module.clone());
    ///
    ///     assert_eq!(wrapped.lock().await.name().await, module.name().await);
    /// }
    /// ```
    pub fn new<M: 'static + SubscanModuleInterface>(module: M) -> Mutex<Self> {
        Mutex::new(Self(Box::new(module)))
    }
    pub async fn run(&mut self, domain: String) {
        self.0.run(domain).await;
    }
    pub async fn name(&self) -> String {
        self.0.name().await
    }
}
