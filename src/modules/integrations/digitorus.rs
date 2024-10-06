use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
};
use reqwest::Url;
use serde_json::Value;

pub const DIGITORUS_MODULE_NAME: &str = "digitorus";
pub const DIGITORUS_URL: &str = "https://certificatedetails.com";
pub const DIGITORUS_SUBDOMAIN_TAG: &str = "main > div:nth-last-child(3) > div > div > a";

/// `Digitorus` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                          |
/// |:------------------:|:----------------------------------------------:|
/// | Module Name        | `digitorus`                                    |
/// | Doc URL            | <https://certificatedetails.com>               |
/// | Subdomain Selector | `main > div:nth-last-child(3) > div > div > a` |
/// | Authentication     | [`APIAuthMethod::NoAuth`]                      |
/// | Requester          | [`HTTPClient`]                                 |
/// | Extractor          | [`HTMLExtractor`]                              |
pub struct Digitorus {}

impl Digitorus {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let selector: String = DIGITORUS_SUBDOMAIN_TAG.into();
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: HTMLExtractor = HTMLExtractor::new(selector, vec![]);

        let generic = GenericIntegrationModule {
            name: DIGITORUS_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{DIGITORUS_URL}/{domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }
}
