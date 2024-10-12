use crate::{
    enums::{AuthenticationMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::regex::RegexExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{core::SubscanModuleCoreComponents, func::GenericIntegrationCoreFuncs},
};
use reqwest::Url;
use serde_json::Value;

pub const HACKERTARGET_MODULE_NAME: &str = "hackertarget";
pub const HACKERTARGET_URL: &str = "https://api.hackertarget.com/hostsearch";

/// `HackerTarget` integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `hackertarget`                             |
/// | Doc URL            | <https://hackertarget.com>                 |
/// | Authentication     | [`AuthenticationMethod::NoAuthentication`] |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`RegexExtractor`]                         |
pub struct HackerTarget {}

impl HackerTarget {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: RegexExtractor = RegexExtractor::default();

        let generic = GenericIntegrationModule {
            name: HACKERTARGET_MODULE_NAME.into(),
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
        format!("{HACKERTARGET_URL}/?q={domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }
}
