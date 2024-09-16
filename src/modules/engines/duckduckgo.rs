use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const DUCKDUCKGO_MODULE_NAME: &str = "DuckDuckGo";
const DUCKDUCKGO_SEARCH_URL: &str = "https://duckduckgo.com";
const DUCKDUCKGO_SEARCH_PARAM: &str = "q";
const DUCKDUCKGO_CITE_TAG: &str = "article > div > div > a > span:first-child";

/// DuckDuckGo search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                        |
/// |:------------------:|:--------------------------------------------:|
/// | Module Name        | `DuckDuckGo`                                 |
/// | Search URL         | <https://duckduckgo.com>                    |
/// | Search Param       | `q`                                          |
/// | Subdomain Selector | `article > div > div > a > span:first-child` |
pub struct DuckDuckGo {}

impl<'a> DuckDuckGo {
    /// Create a new [`DuckDuckGo`] module instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::engines::duckduckgo;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let duckduckgo = duckduckgo::DuckDuckGo::new();
    ///
    ///     // do something with duckduckgo instance
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericSearchEngineModule<'a> {
        let extractor = HTMLExtractor::new(String::from(DUCKDUCKGO_CITE_TAG), vec![]);

        GenericSearchEngineModule {
            name: String::from(DUCKDUCKGO_MODULE_NAME),
            url: Url::parse(DUCKDUCKGO_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(DUCKDUCKGO_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::ChromeBrowser),
            extractor: extractor.into(),
        }
    }
}
