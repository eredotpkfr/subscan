use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
    utils::regex::generate_subdomain_regex,
};
use regex::Match;
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const BUFFEROVER_MODULE_NAME: &str = "bufferover";
pub const BUFFEROVER_URL: &str = "https://tls.bufferover.run";

/// `BufferOver` API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                             |
/// |:------------------:|:---------------------------------:|
/// | Module Name        | `bufferover`                      |
/// | Doc URL            | <https://tls.bufferover.run>      |
/// | Authentication     | [`APIAuthMethod::APIKeyAsHeader`] |
pub struct BufferOver {}

impl BufferOver {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericAPIIntegrationModule {
            name: BUFFEROVER_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::APIKeyAsHeader("X-API-Key".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{BUFFEROVER_URL}/dns?q={domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: String) -> BTreeSet<Subdomain> {
        let mut subs = BTreeSet::new();
        let pattern = generate_subdomain_regex(domain).unwrap();

        if let Some(results) = content["Results"].as_array() {
            let filter = |item: &Value| {
                let to_string = |matches: Match| matches.as_str().to_string();

                pattern.find(item.as_str()?).map(to_string)
            };

            subs.extend(results.iter().filter_map(filter));
        }

        subs
    }
}
