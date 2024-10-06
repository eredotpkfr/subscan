use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const CHAOS_MODULE_NAME: &str = "chaos";
pub const CHAOS_URL: &str = "https://dns.projectdiscovery.io/dns";

/// `Chaos` API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                               |
/// |:------------------:|:-----------------------------------:|
/// | Module Name        | `chaos`                             |
/// | Doc URL            | <https://cloud.projectdiscovery.io> |
/// | Authentication     | [`APIAuthMethod::APIKeyAsHeader`]   |
/// | Requester          | [`HTTPClient`]                      |
/// | Extractor          | [`JSONExtractor`]                   |
pub struct Chaos {}

impl Chaos {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericAPIIntegrationModule {
            name: CHAOS_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::APIKeyAsHeader("Authorization".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{CHAOS_URL}/{domain}/subdomains")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: String) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["subdomains"].as_array() {
            let filter = |item: &Value| Some(format!("{}.{}", item.as_str()?, domain));

            return subs.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
