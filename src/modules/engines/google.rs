use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const GOOGLE_MODULE_NAME: &str = "Google";
const GOOGLE_SEARCH_URL: &str = "https://www.google.com/search";
const GOOGLE_SEARCH_PARAM: &str = "q";
const GOOGLE_CITE_TAG: &str = "cite";

/// Google search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                           |
/// |:------------------:|:-------------------------------:|
/// | Module Name        | `Google`                        |
/// | Search URL         | <https://www.google.com/search> |
/// | Search Param       | `q`                             |
/// | Subdomain Selector | `cite`                          |
pub struct Google {}

impl<'a> Google {
    /// Create a new [`Google`] module instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::engines::google;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let google = google::Google::new();
    ///
    ///     // do something with google instance
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericSearchEngineModule<'a> {
        let extractor = HTMLExtractor::new(String::from(GOOGLE_CITE_TAG), vec![]);

        GenericSearchEngineModule {
            name: String::from(GOOGLE_MODULE_NAME),
            url: Url::parse(GOOGLE_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(GOOGLE_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: extractor.into(),
        }
    }
}
