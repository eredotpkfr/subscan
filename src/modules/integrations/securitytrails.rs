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

pub const SECURITYTRAILS_MODULE_NAME: &str = "securitytrails";
pub const SECURITYTRAILS_URL: &str = "https://api.securitytrails.com/v1/domain";

/// `SecurityTrails` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `securitytrails`                           |
/// | Doc URL            | <https://securitytrails.com>               |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsHeader`]   |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`JSONExtractor`]                          |
/// | Generic            | [`GenericIntegrationModule`]               |
pub struct SecurityTrails {}

impl SecurityTrails {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: SECURITYTRAILS_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsHeader("APIKEY".into()),
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
        format!("{SECURITYTRAILS_URL}/{domain}/subdomains")
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: &str) -> Result<BTreeSet<Subdomain>> {
        if let Some(subdomains) = content["subdomains"].as_array() {
            let filter = |item: &Value| Some(format!("{}.{domain}", item.as_str()?));

            return Ok(subdomains.iter().filter_map(filter).collect());
        }

        Err(JSONExtract.into())
    }
}
