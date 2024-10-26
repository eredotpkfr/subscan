use crate::enums::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Core subdomain data type
pub type Subdomain = String;
/// `Subscan`` module type wrapper
pub type SubscanModule = Arc<Mutex<SubscanModuleDispatcher>>;

impl From<SubscanModuleDispatcher> for SubscanModule {
    fn from(module: SubscanModuleDispatcher) -> Self {
        Self::new(Mutex::new(module))
    }
}

/// Container for core components of `Subscan` modules
pub struct SubscanModuleCoreComponents {
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
}
