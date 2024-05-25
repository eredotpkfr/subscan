use crate::extractors::html::HTMLExtractor;

use crate::modules::generics::searchengine::GenericSearchEngineModule;
use crate::requesters::client::HTTPClient;
use crate::SearchQueryParam;
use reqwest::Url;

const BING_MODULE_NAME: &str = "Bing";
const BING_SEARCH_URL: &str = "https://www.bing.com/search";
const BING_SEARCH_PARAM: &str = "q";
const BING_CITE_TAG: &str = "cite";

pub struct Bing {}

impl Bing {
    pub fn new() -> GenericSearchEngineModule {
        GenericSearchEngineModule {
            name: String::from(BING_MODULE_NAME),
            url: Url::parse(BING_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(BING_SEARCH_PARAM),
            requester: Box::new(HTTPClient::new()),
            extractor: Box::new(HTMLExtractor::new(String::from(BING_CITE_TAG), vec![])),
        }
    }
}
