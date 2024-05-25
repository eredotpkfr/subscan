use crate::extractors::html::HTMLExtractor;

use crate::modules::generics::searchengine::GenericSearchEngineModule;
use crate::requesters::client::HTTPClient;
use crate::SearchQueryParam;
use reqwest::Url;

const YAHOO_MODULE_NAME: &str = "Yahoo";
const YAHOO_SEARCH_URL: &str = "https://search.yahoo.com/search";
const YAHOO_SEARCH_PARAM: &str = "p";
const YAHOO_CITE_TAG: &str = "ol > li > div > div > h3 > a > span";

pub struct Yahoo {}

impl Yahoo {
    pub fn new() -> GenericSearchEngineModule {
        GenericSearchEngineModule {
            name: String::from(YAHOO_MODULE_NAME),
            url: Url::parse(YAHOO_SEARCH_URL).expect("URL parse error!"),
            param: SearchQueryParam::from(YAHOO_SEARCH_PARAM),
            requester: Box::new(HTTPClient::new()),
            extractor: Box::new(HTMLExtractor::new(
                String::from(YAHOO_CITE_TAG),
                vec!["<b>".to_string(), "</b>".to_string()],
            )),
        }
    }
}
