use crate::enums::{Content, RequesterDispatcher};

use super::core::Subdomain;
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;
use tokio::sync::MutexGuard;

/// Inner extract method type definition for [`JSONExtractor`](crate::extractors::json::JSONExtractor)
/// In summary it takes a [`Value`] as a parameter and parse subdomains
pub type InnerExtractFunc = Box<dyn Fn(Value, &str) -> BTreeSet<Subdomain> + Sync + Send>;
/// Get query url function, [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses this type to get start query URL
pub type GetQueryUrlFunc = Box<dyn Fn(&str) -> String + Sync + Send>;
/// Get next url function, [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses this function to get next query URL for fetch API fully
pub type GetNextUrlFunc = Box<dyn Fn(Url, Value) -> Option<Url> + Sync + Send>;
/// Custom requester function that allows to handle [`RequesterDispatcher`] instance and make
/// custom requests like a posting data, custom authentications, modifying query URL etc.
/// If sets [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses this function by default otherwise uses simple [`get_request`](crate::interfaces::requester::RequesterInterface::get_request)
/// method
pub type CustomRequestFunc =
    Box<dyn Fn(&MutexGuard<RequesterDispatcher>, Url) -> Content + Sync + Send>;

pub struct GenericIntegrationCoreFuncs {
    pub url: GetQueryUrlFunc,
    pub next: GetNextUrlFunc,
    pub request: Option<CustomRequestFunc>,
}
