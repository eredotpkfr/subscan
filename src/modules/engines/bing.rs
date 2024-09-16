use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const BING_MODULE_NAME: &str = "Bing";
const BING_SEARCH_URL: &str = "https://www.bing.com/search";
const BING_SEARCH_PARAM: &str = "q";
const BING_CITE_TAG: &str = "cite";

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

impl<'a> Bing {
    /// Create a new [`Bing`] module instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::engines::bing;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let bing = bing::Bing::new();
    ///
    ///     // do something with bing instance
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericSearchEngineModule<'a> {
        let extractor = HTMLExtractor::new(String::from(BING_CITE_TAG), vec![]);

        GenericSearchEngineModule {
            name: String::from(BING_MODULE_NAME),
            url: Url::parse(BING_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(BING_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: extractor.into(),
        }
    }
}
