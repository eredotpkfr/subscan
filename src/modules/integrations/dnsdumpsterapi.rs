use std::collections::BTreeSet;

use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    error::ModuleErrorKind::JSONExtractError,
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Result, Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
};
use reqwest::Url;
use serde_json::Value;

pub const DNSDUMPSTERAPI_MODULE_NAME: &str = "dnsdumpsterapi";
pub const DNSDUMPSTERAPI_URL: &str = "https://api.dnsdumpster.com/domain";

/// `DNSDumpsterAPI` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `dnsdumpsterapi`                           |
/// | Doc URL            | <https://dnsdumpster.com>                  |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsHeader`]   |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`JSONExtractor`]                          |
/// | Generic            | [`GenericIntegrationModule`]               |
pub struct DNSDumpsterAPI {}

impl DNSDumpsterAPI {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: DNSDUMPSTERAPI_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsHeader("X-API-Key".into()),
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
        format!("{DNSDUMPSTERAPI_URL}/{domain}")
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: &str) -> Result<BTreeSet<Subdomain>> {
        if let Some(a_records) = content["a"].as_array() {
            let filter = |item: &Value| Some(item["host"].as_str()?.to_string());

            return Ok(a_records.iter().filter_map(filter).collect());
        }

        Err(JSONExtractError.into())
    }
}
