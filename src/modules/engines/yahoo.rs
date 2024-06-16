use crate::{
    cache::requesters, enums::RequesterType, extractors::html::HTMLExtractor,
    modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
};
use reqwest::Url;

const YAHOO_MODULE_NAME: &str = "Yahoo";
const YAHOO_SEARCH_URL: &str = "https://search.yahoo.com/search";
const YAHOO_SEARCH_PARAM: &str = "p";
const YAHOO_CITE_TAG: &str = "ol > li > div > div > h3 > a > span";

pub struct Yahoo {}

impl<'a> Yahoo {
    pub fn new() -> GenericSearchEngineModule<'a> {
        GenericSearchEngineModule {
            name: String::from(YAHOO_MODULE_NAME),
            url: Url::parse(YAHOO_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(YAHOO_SEARCH_PARAM),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: Box::new(HTMLExtractor::new(
                String::from(YAHOO_CITE_TAG),
                vec!["<b>".to_string(), "</b>".to_string()],
            )),
        }
    }
}
