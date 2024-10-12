use std::collections::BTreeSet;

use crate::{
    enums::{AuthenticationMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
};
use reqwest::Url;
use serde_json::Value;

pub const SUBDOMAINCENTER_MODULE_NAME: &str = "subdomaincenter";
pub const SUBDOMAINCENTER_URL: &str = "https://api.subdomain.center";

/// `SubdomainCenter` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `subdomaincenter`                          |
/// | Doc URL            | <https://www.subdomain.center>             |
/// | Authentication     | [`AuthenticationMethod::NoAuthentication`] |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`JSONExtractor`]                          |
pub struct SubdomainCenter {}

impl SubdomainCenter {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: SUBDOMAINCENTER_MODULE_NAME.into(),
            auth: AuthenticationMethod::NoAuthentication,
            funcs: GenericIntegrationCoreFuncs {
                url: Box::new(Self::get_query_url),
                next: Box::new(Self::get_next_url),
            },
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{SUBDOMAINCENTER_URL}/?domain={domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: &str) -> BTreeSet<Subdomain> {
        if let Some(passives) = content.as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            return passives.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
