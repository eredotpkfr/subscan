use crate::extractors::html::HTMLExtractor;
use crate::interfaces::module::SubscanModuleInterface;
use crate::modules::generics::searchengine::GenericSearchEngineModule;
use reqwest::Url;

const BING_MODULE_NAME: &str = "Bing";
const BING_SEARCH_URL: &str = "https://www.bing.com/search";
const BING_SEARCH_PARAM: &str = "q";
const BING_CITE_TAG: &str = "cite";

pub struct Bing {}

impl Bing {
    pub fn new() -> Box<dyn SubscanModuleInterface> {
        let name = String::from(BING_MODULE_NAME);
        let url = Url::parse(BING_SEARCH_URL).expect("URL parse error!");
        let query_param = String::from(BING_SEARCH_PARAM);
        let extractor = Box::new(HTMLExtractor::new(String::from(BING_CITE_TAG), vec![]));

        GenericSearchEngineModule::new(name, url, query_param, extractor)
    }
}
