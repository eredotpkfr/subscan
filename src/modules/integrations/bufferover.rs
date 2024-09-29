use crate::{
    enums::{APIAuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
    utils::regex::generate_subdomain_regex,
};
use regex::Match;
use serde_json::Value;
use std::collections::BTreeSet;

/// Bufferover API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Bufferover {}

pub const BUFFEROVER_MODULE_NAME: &str = "Bufferover";
pub const BUFFEROVER_URL: &str = "https://tls.bufferover.run";

impl Bufferover {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: BUFFEROVER_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(move |_, _| None),
            auth: APIAuthMethod::APIKeyAsHeader("X-API-Key".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{BUFFEROVER_URL}/dns?q={domain}")
    }

    pub fn extract(content: Value, domain: String) -> BTreeSet<Subdomain> {
        let pattern = generate_subdomain_regex(domain).unwrap();

        if let Some(subs) = content["Results"].as_array() {
            let filter = |item: &Value| {
                let line = item.as_str()?.to_string();
                let to_string = |matches: Match| matches.as_str().to_string();

                pattern.find(&line).map(to_string)
            };

            subs.iter().filter_map(filter).collect()
        } else {
            BTreeSet::new()
        }
    }
}
