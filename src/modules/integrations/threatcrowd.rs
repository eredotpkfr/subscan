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

pub const THREATCROWD_MODULE_NAME: &str = "threatcrowd";
pub const THREATCROWD_URL: &str = "http://ci-www.threatcrowd.org/searchApi/v2/domain/report";

/// `ThreatCrowd` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                           |
/// |:------------------:|:-------------------------------:|
/// | Module Name        | `threatcrowd`                   |
/// | Doc URL            | <http://ci-www.threatcrowd.org> |
/// | Authentication     | [`APIAuthMethod::NoAuth`]       |
/// | Requester          | [`HTTPClient`]                  |
/// | Extractor          | [`JSONExtractor`]               |
pub struct ThreatCrowd {}

impl ThreatCrowd {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: THREATCROWD_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{THREATCROWD_URL}/?domain={domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(passives) = content["subdomains"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            return passives.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
