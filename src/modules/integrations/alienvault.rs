use std::collections::BTreeSet;

use crate::{
    enums::{APIAuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use serde_json::Value;

/// Alienvault API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct AlienVault {}

pub const ALIENVAULT_MODULE_NAME: &str = "AlienVault";
pub const ALIENVAULT_URL: &str = "https://otx.alienvault.com/api/v1/indicators/domain";

impl AlienVault {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: ALIENVAULT_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(move |_, _| None),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{ALIENVAULT_URL}/{domain}/passive_dns")
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(passives) = content["passive_dns"].as_array() {
            let filter = |item: &Value| Some(item["hostname"].as_str()?.to_string());

            passives.iter().filter_map(filter).collect()
        } else {
            BTreeSet::new()
        }
    }
}
