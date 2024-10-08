use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
};
use reqwest::Url;
use serde_json::Value;

pub const SITEDOSSIER_MODULE_NAME: &str = "sitedossier";
pub const SITEDOSSIER_URL: &str = "http://www.sitedossier.com/parentdomain";
pub const SITEDOSSIER_SUBDOMAIN_TAG: &str = "ol > li > a";

/// `Sitedossier` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                        |
/// |:------------------:|:----------------------------:|
/// | Module Name        | `sitedossier`                |
/// | Doc URL            | <http://www.sitedossier.com> |
/// | Subdomain Selector | `ol > li > a`                |
/// | Authentication     | [`APIAuthMethod::NoAuth`]    |
/// | Requester          | [`HTTPClient`]               |
/// | Extractor          | [`HTMLExtractor`]            |
pub struct Sitedossier {}

impl Sitedossier {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let selector: String = SITEDOSSIER_SUBDOMAIN_TAG.into();
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: HTMLExtractor = HTMLExtractor::new(selector, vec![]);

        let generic = GenericIntegrationModule {
            name: SITEDOSSIER_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{SITEDOSSIER_URL}/{domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }
}