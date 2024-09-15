use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const DUCKDUCKGO_MODULE_NAME: &str = "DuckDuckGo";
const DUCKDUCKGO_SEARCH_URL: &str = "https://duckduckgo.com/";
const DUCKDUCKGO_SEARCH_PARAM: &str = "q";
const DUCKDUCKGO_CITE_TAG: &str = "article > div > div > a > span:first-child";

pub struct DuckDuckGo {}

impl<'a> DuckDuckGo {
    pub fn new() -> GenericSearchEngineModule<'a> {
        let extractor = HTMLExtractor::new(String::from(DUCKDUCKGO_CITE_TAG), vec![]);

        GenericSearchEngineModule {
            name: String::from(DUCKDUCKGO_MODULE_NAME),
            url: Url::parse(DUCKDUCKGO_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(DUCKDUCKGO_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::ChromeBrowser),
            extractor: extractor.try_into().unwrap(),
        }
    }
}
