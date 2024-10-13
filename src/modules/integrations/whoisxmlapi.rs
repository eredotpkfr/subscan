use std::collections::BTreeSet;

use crate::{
    enums::{AuthenticationMethod, Content, RequesterDispatcher, SubscanModuleDispatcher},
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

pub const WHOISXMLAPI_MODULE_NAME: &str = "whoisxmlapi";
pub const WHOISXMLAPI_URL: &str = "https://subdomains.whoisxmlapi.com/api/v1";

/// `WhoisXMLAPI` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                        |
/// |:------------------:|:--------------------------------------------:|
/// | Module Name        | `whoisxmlapi`                                |
/// | Doc URL            | <https://www.whoisxmlapi.com>                |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsQueryParam`] |
/// | Requester          | [`HTTPClient`]                               |
/// | Extractor          | [`JSONExtractor`]                            |
/// | Is Generic?        | [`GenericIntegrationModule`]                 |
pub struct WhoisXMLAPI {}

impl WhoisXMLAPI {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: WHOISXMLAPI_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsQueryParam("apiKey".into()),
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
        format!("{WHOISXMLAPI_URL}/?domainName={domain}")
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: &str) -> BTreeSet<Subdomain> {
        if let Some(passives) = content["result"]["records"].as_array() {
            let filter = |item: &Value| Some(item["domain"].as_str()?.to_string());

            return passives.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
