use crate::{
    enums::{RequesterDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    modules::generics::engine::GenericSearchEngineModule,
    requesters::client::HTTPClient,
};
use reqwest::Url;

pub const GOOGLE_MODULE_NAME: &str = "google";
pub const GOOGLE_SEARCH_URL: &str = "https://www.google.com/search";
pub const GOOGLE_SEARCH_PARAM: &str = "q";
pub const GOOGLE_CITE_TAG: &str = "cite";

/// Google search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                           |
/// |:------------------:|:-------------------------------:|
/// | Module Name        | `google`                        |
/// | Search URL         | <https://www.google.com/search> |
/// | Search Param       | `q`                             |
/// | Subdomain Selector | `cite`                          |
/// | Requester          | [`HTTPClient`]                  |
/// | Extractor          | [`HTMLExtractor`]               |
pub struct Google {}

impl Google {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let extractor: HTMLExtractor = HTMLExtractor::new(GOOGLE_CITE_TAG.into(), vec![]);
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let url = Url::parse(GOOGLE_SEARCH_URL);

        let generic = GenericSearchEngineModule {
            name: GOOGLE_MODULE_NAME.into(),
            param: GOOGLE_SEARCH_PARAM.into(),
            url: url.unwrap(),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }
}
