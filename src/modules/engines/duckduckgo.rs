use crate::{
    enums::{RequesterDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    modules::generics::search_engine::GenericSearchEngineModule,
    requesters::chrome::ChromeBrowser,
};
use reqwest::Url;

pub const DUCKDUCKGO_MODULE_NAME: &str = "DuckDuckGo";
pub const DUCKDUCKGO_SEARCH_URL: &str = "https://duckduckgo.com";
pub const DUCKDUCKGO_SEARCH_PARAM: &str = "q";
pub const DUCKDUCKGO_CITE_TAG: &str = "article > div > div > a > span:first-child";

/// DuckDuckGo search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                        |
/// |:------------------:|:--------------------------------------------:|
/// | Module Name        | `DuckDuckGo`                                 |
/// | Search URL         | <https://duckduckgo.com>                     |
/// | Search Param       | `q`                                          |
/// | Subdomain Selector | `article > div > div > a > span:first-child` |
pub struct DuckDuckGo {}

impl DuckDuckGo {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let extractor: HTMLExtractor = HTMLExtractor::new(DUCKDUCKGO_CITE_TAG.into(), vec![]);
        let requester: RequesterDispatcher = ChromeBrowser::default().into();
        let url = Url::parse(DUCKDUCKGO_SEARCH_URL);

        let generic = GenericSearchEngineModule {
            name: DUCKDUCKGO_MODULE_NAME.into(),
            param: DUCKDUCKGO_SEARCH_PARAM.into(),
            url: url.unwrap(),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }
}
