use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const VIRUSTOTAL_MODULE_NAME: &str = "virustotal";
pub const VIRUSTOTAL_URL: &str = "https://www.virustotal.com/api/v3/domains";

/// `VirusTotal` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                        |
/// |:------------------:|:--------------------------------------------:|
/// | Module Name        | `virustotal`                                 |
/// | Doc URL            | <https://www.virustotal.com/gui/home/upload> |
/// | Authentication     | [`APIAuthMethod::APIKeyAsHeader`]            |
/// | Requester          | [`HTTPClient`]                               |
/// | Extractor          | [`JSONExtractor`]                            |
pub struct VirusTotal {}

impl VirusTotal {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: VIRUSTOTAL_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::APIKeyAsHeader("X-APIKey".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{VIRUSTOTAL_URL}/{domain}/subdomains?limit=250")
    }

    pub fn get_next_url(_url: Url, content: Value) -> Option<Url> {
        if let Some(next_url) = content["links"]["next"].as_str() {
            Url::parse(next_url).ok()
        } else {
            None
        }
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(passives) = content["data"].as_array() {
            let filter = |item: &Value| Some(item["id"].as_str()?.to_string());

            return passives.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
