use crate::{
    enums::{APIAuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

/// Binaryedge API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Binaryedge {}

pub const BINARYEDGE_MODULE_NAME: &str = "Binaryedge";
pub const BINARYEDGE_URL: &str = "https://api.binaryedge.io/v2/query/domains/subdomain";

impl Binaryedge {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: BINARYEDGE_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::APIKeyAsHeader("X-Key".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{BINARYEDGE_URL}/{domain}")
    }

    pub fn get_next_url(mut url: Url, _content: Value) -> Option<Url> {
        let page_param = url.query_pairs().find(|item| item.0 == "page");

        if let Some(page) = page_param {
            let new_page = page.1.parse::<usize>().unwrap() + 1;

            url.set_query(Some(&format!("page={new_page}")));
        } else {
            url.set_query(Some("page=2"));
        }

        Some(url)
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["events"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            subs.iter().filter_map(filter).collect()
        } else {
            BTreeSet::new()
        }
    }
}
