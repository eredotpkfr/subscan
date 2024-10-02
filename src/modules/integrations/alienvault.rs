use std::collections::BTreeSet;

use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;

pub const ALIENVAULT_MODULE_NAME: &str = "AlienVault";
pub const ALIENVAULT_URL: &str = "https://otx.alienvault.com/api/v1/indicators/domain";

/// Alienvault API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct AlienVault {}

impl AlienVault {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericAPIIntegrationModule {
            name: ALIENVAULT_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{ALIENVAULT_URL}/{domain}/passive_dns")
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(passives) = content["passive_dns"].as_array() {
            let filter = |item: &Value| Some(item["hostname"].as_str()?.to_string());

            return passives.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
