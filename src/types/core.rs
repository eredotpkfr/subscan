use crate::enums::{RequesterDispatcher, SubdomainExtractorDispatcher};
use tokio::sync::Mutex;

/// Core subdomain data type
pub type Subdomain = String;

/// Container for core components of `Subscan` modules
pub struct SubscanModuleCoreComponents {
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
}
