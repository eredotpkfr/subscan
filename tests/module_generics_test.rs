mod common;

#[cfg(test)]
mod searchengine {
    use super::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_MODULE_NAME, TEST_URL};
    use reqwest::Url;
    use std::collections::BTreeSet;
    use subscan::{
        cache::requesters, enums::RequesterType, extractors::regex::RegexExtractor,
        interfaces::module::SubscanModuleInterface,
        modules::generics::searchengine::GenericSearchEngineModule, types::query::SearchQueryParam,
    };

    #[tokio::test]
    async fn get_search_query_test() {
        let module = GenericSearchEngineModule {
            name: TEST_MODULE_NAME.to_string(),
            url: Url::parse(TEST_URL).unwrap(),
            param: SearchQueryParam::from("q"),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: RegexExtractor::default().into(),
        };

        let mut query = module.get_search_query(TEST_DOMAIN.to_string()).await;

        assert_eq!(query.as_search_str(), "site:foo.com");
        assert_eq!(module.name().await, "foo-module");
    }

    #[tokio::test]
    #[stubr::mock("module/generics/search-engine.json")]
    async fn run_test() {
        let mut module = GenericSearchEngineModule {
            name: TEST_MODULE_NAME.to_string(),
            url: Url::parse(&stubr.path("/search")).unwrap(),
            param: SearchQueryParam::from("q"),
            requester: requesters::get_by_type(&RequesterType::HTTPClient),
            extractor: RegexExtractor::default().into(),
        };

        let result = module.run(TEST_DOMAIN.to_string()).await;

        assert_eq!(module.name().await, "foo-module");
        assert_eq!(result, BTreeSet::from([String::from(TEST_BAR_SUBDOMAIN)]));
    }
}
