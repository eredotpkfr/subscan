use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const YAHOO_MODULE_NAME: &str = "Yahoo";
const YAHOO_SEARCH_URL: &str = "https://search.yahoo.com/search";
const YAHOO_SEARCH_PARAM: &str = "p";
const YAHOO_CITE_TAG: &str = "ol > li > div > div > h3 > a > span";

/// Yahoo search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                 |
/// |:------------------:|:-------------------------------------:|
/// | Module Name        | `Yahoo`                               |
/// | Search URL         | <https://search.yahoo.com/search>     |
/// | Search Param       | `p`                                   |
/// | Subdomain Selector | `ol > li > div > div > h3 > a > span` |
pub struct Yahoo {}

impl<'a> Yahoo {
    /// Create a new [`Yahoo`] module instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::engines::yahoo;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let yahoo = yahoo::Yahoo::new();
    ///
    ///     // do something with yahoo instance
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericSearchEngineModule<'a> {
        let extractor = HTMLExtractor::new(
            String::from(YAHOO_CITE_TAG),
            vec!["<b>".to_string(), "</b>".to_string()],
        );

        GenericSearchEngineModule {
            name: String::from(YAHOO_MODULE_NAME),
            url: Url::parse(YAHOO_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(YAHOO_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: extractor.into(),
        }
    }
}
