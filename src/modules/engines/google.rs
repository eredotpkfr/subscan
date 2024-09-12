use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const GOOGLE_MODULE_NAME: &str = "Google";
const GOOGLE_SEARCH_URL: &str = "https://www.google.com/search";
const GOOGLE_SEARCH_PARAM: &str = "q";
const GOOGLE_CITE_TAG: &str = "cite";

pub struct Google {}

impl<'a> Google {
    pub fn new() -> GenericSearchEngineModule<'a> {
        GenericSearchEngineModule {
            name: String::from(GOOGLE_MODULE_NAME),
            url: Url::parse(GOOGLE_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(GOOGLE_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: Box::new(HTMLExtractor::new(String::from(GOOGLE_CITE_TAG), vec![])),
        }
    }
}
