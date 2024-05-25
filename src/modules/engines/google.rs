use crate::extractors::html::HTMLExtractor;
use crate::interfaces::module::SubscanModuleInterface;
use crate::modules::generics::searchengine::GenericSearchEngineModule;
use crate::requesters::client::HTTPClient;
use crate::types::query::SearchQueryParam;
use reqwest::Url;

const GOOGLE_MODULE_NAME: &str = "Google";
const GOOGLE_SEARCH_URL: &str = "https://www.google.com/search";
const GOOGLE_SEARCH_PARAM: &str = "q";
const GOOGLE_CITE_TAG: &str = "cite";

pub struct Google {}

impl Google {
    pub fn new() -> Box<dyn SubscanModuleInterface> {
        let name = String::from(GOOGLE_MODULE_NAME);
        let url = Url::parse(GOOGLE_SEARCH_URL).expect("URL parse error!");
        let param = SearchQueryParam::from(GOOGLE_SEARCH_PARAM);
        let extractor = Box::new(HTMLExtractor::new(String::from(GOOGLE_CITE_TAG), vec![]));
        let requester = Box::new(HTTPClient::new());

        Box::new(GenericSearchEngineModule::new(
            name, url, param, requester, extractor,
        ))
    }
}
