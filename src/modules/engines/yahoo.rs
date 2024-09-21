use crate::{
    enums::RequesterDispatcher, extractors::html::HTMLExtractor,
    modules::generics::search_engine::GenericSearchEngineModule, requesters::client::HTTPClient,
};
use reqwest::Url;

pub const YAHOO_MODULE_NAME: &str = "Yahoo";
pub const YAHOO_SEARCH_URL: &str = "https://search.yahoo.com/search";
pub const YAHOO_SEARCH_PARAM: &str = "p";
pub const YAHOO_CITE_TAG: &str = "ol > li > div > div > h3 > a > span";

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

impl Yahoo {
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
    pub fn new() -> GenericSearchEngineModule {
        let removes: Vec<String> = vec!["<b>".into(), "</b>".into()];
        let extractor: HTMLExtractor = HTMLExtractor::new(YAHOO_CITE_TAG.into(), removes);
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let url = Url::parse(YAHOO_SEARCH_URL);

        GenericSearchEngineModule {
            name: YAHOO_MODULE_NAME.into(),
            param: YAHOO_SEARCH_PARAM.into(),
            url: url.unwrap(),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }
}
