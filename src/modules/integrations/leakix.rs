use std::collections::BTreeSet;

use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;

pub const LEAKIX_MODULE_NAME: &str = "leakix";
pub const LEAKIX_URL: &str = "https://leakix.net/api";

/// `Leakix` integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                      |
/// |:------------------:|:--------------------------:|
/// | Module Name        | `leakix`                   |
/// | Doc URL            | <https://leakix.net>       |
/// | Authentication     | [`APIAuthMethod::NoAuth`]  |
/// | Requester          | [`HTTPClient`]             |
/// | Extractor          | [`JSONExtractor`]          |
pub struct Leakix {}

impl Leakix {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: LEAKIX_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{LEAKIX_URL}/subdomains/{domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(subs) = content.as_array() {
            let filter = |item: &Value| Some(item["subdomain"].as_str()?.to_string());

            return subs.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
