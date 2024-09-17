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
        cache::requesters, enums::RequesterType, extractors::regex::RegexExtractor,
        modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
    };

    pub fn generic_search_engine<'a>(url: &str) -> GenericSearchEngineModule<'a> {
        GenericSearchEngineModule {
            name: TEST_MODULE_NAME.to_string(),
            url: Url::parse(url).unwrap(),
            param: SearchQueryParam::from("q"),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: RegexExtractor::default().into(),
        }
    }
}
