use std::collections::BTreeSet;

use reqwest::Url;
use serde_json::Value;

use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    error::ModuleErrorKind::JSONExtract,
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Result, Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
};

pub const LEAKIX_MODULE_NAME: &str = "leakix";
pub const LEAKIX_URL: &str = "https://leakix.net/api";

/// `Leakix` integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `leakix`                                   |
/// | Doc URL            | <https://leakix.net>                       |
/// | Authentication     | [`AuthenticationMethod::NoAuthentication`] |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`JSONExtractor`]                          |
/// | Generic            | [`GenericIntegrationModule`]               |
pub struct Leakix {}

impl Leakix {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: LEAKIX_MODULE_NAME.into(),
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
        format!("{LEAKIX_URL}/subdomains/{domain}")
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: &str) -> Result<BTreeSet<Subdomain>> {
        let filter = |item: &Value| Some(item["subdomain"].as_str()?.to_string());

        if let Some(subdomains) = content.as_array() {
            return Ok(subdomains.iter().filter_map(filter).collect());
        }

        Err(JSONExtract.into())
    }
}
