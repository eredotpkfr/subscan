use crate::{
    enums::RequesterDispatcher, extractors::html::HTMLExtractor,
    modules::generics::search_engine::GenericSearchEngineModule, requesters::client::HTTPClient,
};
use reqwest::Url;

pub const GOOGLE_MODULE_NAME: &str = "Google";
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
/// | Module Name        | `Google`                        |
/// | Search URL         | <https://www.google.com/search> |
/// | Search Param       | `q`                             |
/// | Subdomain Selector | `cite`                          |
pub struct Google {}

impl Google {
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
    pub fn new() -> GenericSearchEngineModule {
        let extractor: HTMLExtractor = HTMLExtractor::new(GOOGLE_CITE_TAG.into(), vec![]);
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let url = Url::parse(GOOGLE_SEARCH_URL);

        GenericSearchEngineModule {
            name: GOOGLE_MODULE_NAME.into(),
            param: GOOGLE_SEARCH_PARAM.into(),
            url: url.unwrap(),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }
}
