use crate::{
    enums::dispatchers::{
        RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher,
    },
    error::SubscanError,
};
use flume::{Receiver, Sender};
use std::{result, sync::Arc};
use tokio::sync::Mutex;

/// Result type
pub type Result<T> = result::Result<T, SubscanError>;

/// Core subdomain data type
pub type Subdomain = String;
/// `Subscan` module type wrapper
pub type SubscanModule = Arc<Mutex<SubscanModuleDispatcher>>;

impl From<SubscanModuleDispatcher> for SubscanModule {
    fn from(module: SubscanModuleDispatcher) -> Self {
        Self::new(Mutex::new(module))
    }
}

/// Flume unbounded channel return tuple
pub type UnboundedFlumeChannelTuple<T> = (Sender<T>, Receiver<T>);

/// Flume unbounded channel with generic typed
pub struct UnboundedFlumeChannel<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

/// Container for core components of `Subscan` modules
pub struct SubscanModuleCoreComponents {
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
}
