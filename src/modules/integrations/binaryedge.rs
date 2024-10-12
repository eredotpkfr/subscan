use crate::{
    enums::{AuthenticationMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
    utils::http,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const BINARYEDGE_MODULE_NAME: &str = "binaryedge";
pub const BINARYEDGE_URL: &str = "https://api.binaryedge.io/v2/query/domains/subdomain";

/// `BinaryEdge` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                    |
/// |:------------------:|:----------------------------------------:|
/// | Module Name        | `binaryedge`                             |
/// | Doc URL            | <https://www.binaryedge.io>              |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsHeader`] |
/// | Requester          | [`HTTPClient`]                           |
/// | Extractor          | [`JSONExtractor`]                        |
pub struct BinaryEdge {}

impl BinaryEdge {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: BINARYEDGE_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: AuthenticationMethod::APIKeyAsHeader("X-Key".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{BINARYEDGE_URL}/{domain}")
    }

    pub fn get_next_url(mut url: Url, _content: Value) -> Option<Url> {
        let page_param = url.query_pairs().find(|item| item.0 == "page");

        if let Some(page) = page_param {
            let new_page = (page.1.parse::<usize>().unwrap() + 1).to_string();

            http::update_url_query(&mut url, "page", &new_page);
        } else {
            http::update_url_query(&mut url, "page", "2");
        }

        Some(url)
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["events"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            return subs.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
