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

pub const BEVIGIL_MODULE_NAME: &str = "Bevigil";
pub const BEVIGIL_URL: &str = "https://osint.bevigil.com/api";

/// Bevigil API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Bevigil {}

impl Bevigil {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericAPIIntegrationModule {
            name: BEVIGIL_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::APIKeyAsHeader("X-Access-Token".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{BEVIGIL_URL}/{domain}/subdomains")
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["subdomains"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            return subs.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
