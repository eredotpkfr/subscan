pub mod constants {
    pub const TEST_MODULE_NAME: &str = "foo-module";
    pub const TEST_URL: &str = "http://foo.com";
    pub const TEST_DOMAIN: &str = "foo.com";
    pub const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";
}

pub mod mocks {
    use super::constants::TEST_MODULE_NAME;
    use reqwest::Url;
    use subscan::{
        enums::RequesterDispatcher, extractors::regex::RegexExtractor,
        modules::generics::searchengine::GenericSearchEngineModule, requesters::client::HTTPClient,
        types::query::SearchQueryParam,
    };

    pub fn generic_search_engine(url: &str) -> GenericSearchEngineModule {
        GenericSearchEngineModule {
            name: TEST_MODULE_NAME.to_string(),
            url: Url::parse(url).unwrap(),
            param: SearchQueryParam::from("q"),
            requester: RequesterDispatcher::HTTPClient(HTTPClient::default()).into(),
            extractor: RegexExtractor::default().into(),
        }
    }
}
