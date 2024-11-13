use super::core::Subdomain;
use crate::enums::content::Content;
use futures::Future;
use hickory_resolver::TokioAsyncResolver;
use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, net::IpAddr, pin::Pin};

/// Inner extract method type definition for [`JSONExtractor`](crate::extractors::json::JSONExtractor)
/// In summary it takes a [`Value`] as a parameter and parse subdomains
pub type InnerExtractFunc = Box<dyn Fn(Value, &str) -> BTreeSet<Subdomain> + Sync + Send>;
/// Get query url function, [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses this type to get start query URL
pub type GetQueryUrlFunc = Box<dyn Fn(&str) -> String + Sync + Send>;
/// Get next url function, [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses this function to get next query URL for fetch API fully
pub type GetNextUrlFunc = Box<dyn Fn(Url, Content) -> Option<Url> + Sync + Send>;
/// IP address resolver function type
pub type AsyncIPResolveFunc = Box<
    dyn Fn(&TokioAsyncResolver, String) -> Pin<Box<dyn Future<Output = Option<IpAddr>> + Send>>
        + Send
        + Sync,
>;

/// Container for generic integration module functions
pub struct GenericIntegrationCoreFuncs {
    pub url: GetQueryUrlFunc,
    pub next: GetNextUrlFunc,
}
