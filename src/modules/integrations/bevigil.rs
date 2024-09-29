use crate::{
    enums::{APIAuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use serde_json::Value;
use std::collections::BTreeSet;

/// Bevigil API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Bevigil {}

pub const BEVIGIL_MODULE_NAME: &str = "Bevigil";
pub const BEVIGIL_URL: &str = "https://osint.bevigil.com/api";

impl Bevigil {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: BEVIGIL_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(move |_, _| None),
            auth: APIAuthMethod::APIKeyAsHeader("X-Access-Token".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{BEVIGIL_URL}/{domain}/subdomains")
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["subdomains"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            subs.iter().filter_map(filter).collect()
        } else {
            BTreeSet::new()
        }
    }
}
