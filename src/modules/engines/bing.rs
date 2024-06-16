use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const BING_MODULE_NAME: &str = "Bing";
const BING_SEARCH_URL: &str = "https://www.bing.com/search";
const BING_SEARCH_PARAM: &str = "q";
const BING_CITE_TAG: &str = "cite";

pub struct Bing {}

impl Bing {
    pub fn new() -> GenericSearchEngineModule<'static> {
        GenericSearchEngineModule {
            name: String::from(BING_MODULE_NAME),
            url: Url::parse(BING_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(BING_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: Box::new(HTMLExtractor::new(String::from(BING_CITE_TAG), vec![])),
        }
    }
}
