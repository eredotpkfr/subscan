use crate::{
    enums::{APIAuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
    utils::{http, regex::generate_subdomain_regex},
};
use regex::Match;
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

/// Censys API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Censys {}

pub const CENSYS_MODULE_NAME: &str = "Censys";
pub const CENSYS_URL: &str = "https://search.censys.io/api/v2/certificates/search";

impl Censys {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: CENSYS_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::APIKeyAsHeader("Authorization".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{CENSYS_URL}?q={domain}")
    }

    pub fn get_next_url(mut url: Url, content: Value) -> Option<Url> {
        if let Some(cursor) = content["result"]["links"]["next"].as_str() {
            http::update_url_query(&mut url, "cursor", cursor);
            Some(url)
        } else {
            None
        }
    }

    pub fn extract(content: Value, domain: String) -> BTreeSet<Subdomain> {
        let mut subs = BTreeSet::new();

        if let Some(hits) = content["result"]["hits"].as_array() {
            let pattern = generate_subdomain_regex(domain).unwrap();

            for result in hits {
                if let Some(names) = result["names"].as_array() {
                    let matches = |item: &Value| {
                        let to_string = |matched: Match| matched.as_str().to_string();

                        pattern.find(item.as_str()?).map(to_string)
                    };

                    subs.extend(names.iter().filter_map(matches));
                }
            }
        }

        subs
    }
}
