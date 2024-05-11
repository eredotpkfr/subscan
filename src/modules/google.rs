use crate::extractors::html::HTMLExtractor;
use crate::interfaces::module::SubscanModuleInterface;
use crate::modules::generics::searchengine::GenericSearchEngineModule;
use reqwest::Url;

pub struct Google {}

impl Google {
    pub fn new() -> Box<dyn SubscanModuleInterface> {
        let name = String::from("Google");
        let url = Url::parse("https://www.google.com/search").expect("URL parse error!");
        let query_param = String::from("q");
        let extractor = Box::new(HTMLExtractor::new("cite".to_string()));

        Box::new(GenericSearchEngineModule {
            name,
            url,
            query_param,
            extractor,
        })
    }
}
