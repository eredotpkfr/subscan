use crate::{
    enums::{APIAuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use serde_json::Value;
use std::collections::BTreeSet;

/// Anubis API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Anubis {}

pub const ANUBIS_MODULE_NAME: &str = "Anubis";
pub const ANUBIS_URL: &str = "https://jonlu.ca/anubis/subdomains";

impl Anubis {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: ANUBIS_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{ANUBIS_URL}/{domain}")
    }

    pub fn extract(content: Value) -> BTreeSet<Subdomain> {
        if let Some(subs) = content.as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            BTreeSet::from_iter(subs.iter().filter_map(filter))
        } else {
            BTreeSet::new()
        }
    }
}
