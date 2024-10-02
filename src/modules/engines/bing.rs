use crate::{
    enums::{RequesterDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    modules::generics::search_engine::GenericSearchEngineModule,
    requesters::client::HTTPClient,
};
use reqwest::Url;

pub const BING_MODULE_NAME: &str = "Bing";
pub const BING_SEARCH_URL: &str = "https://www.bing.com/search";
pub const BING_SEARCH_PARAM: &str = "q";
pub const BING_CITE_TAG: &str = "cite";

/// Bing search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                         |
/// |:------------------:|:-----------------------------:|
/// | Module Name        | `Bing`                        |
/// | Search URL         | <https://www.bing.com/search> |
/// | Search Param       | `q`                           |
/// | Subdomain Selector | `cite`                        |
pub struct Bing {}

impl Bing {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let extractor: HTMLExtractor = HTMLExtractor::new(BING_CITE_TAG.into(), vec![]);
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let url = Url::parse(BING_SEARCH_URL);

        let generic = GenericSearchEngineModule {
            name: BING_MODULE_NAME.into(),
            param: BING_SEARCH_PARAM.into(),
            url: url.unwrap(),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }
}
