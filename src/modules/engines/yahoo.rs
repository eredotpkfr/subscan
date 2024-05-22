use crate::extractors::html::HTMLExtractor;
use crate::interfaces::module::SubscanModuleInterface;
use crate::modules::generics::searchengine::GenericSearchEngineModule;
use crate::QueryParam;
use reqwest::{Client, Url};

const YAHOO_MODULE_NAME: &str = "Yahoo";
const YAHOO_SEARCH_URL: &str = "https://search.yahoo.com/search";
const YAHOO_SEARCH_PARAM: &str = "p";
const YAHOO_CITE_TAG: &str = "ol > li > div > div > h3 > a > span";

pub struct Yahoo {}

impl Yahoo {
    pub fn new() -> Box<dyn SubscanModuleInterface> {
        let name = String::from(YAHOO_MODULE_NAME);
        let url = Url::parse(YAHOO_SEARCH_URL).expect("URL parse error!");
        let param = QueryParam::from(YAHOO_SEARCH_PARAM);
        let extractor = Box::new(HTMLExtractor::new(
            String::from(YAHOO_CITE_TAG),
            vec!["<b>".to_string(), "</b>".to_string()],
        ));
        let requester = Box::new(Client::new());

        Box::new(GenericSearchEngineModule::new(
            name, url, param, requester, extractor,
        ))
    }
}
