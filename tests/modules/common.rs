pub mod constants {
    pub const TEST_MODULE_NAME: &str = "foo-module";
    pub const TEST_URL: &str = "http://foo.com";
    pub const TEST_DOMAIN: &str = "foo.com";
    pub const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";
    pub const TEST_BAZ_SUBDOMAIN: &str = "baz.foo.com";
}

pub mod mocks {
    use super::constants::TEST_MODULE_NAME;
    use reqwest::Url;
    use serde_json::Value;
    use std::collections::BTreeSet;
    use subscan::{
        enums::RequesterDispatcher,
        extractors::{json::JSONExtractor, regex::RegexExtractor},
        modules::generics::{
            api_integration::GenericAPIIntegrationModule, search_engine::GenericSearchEngineModule,
        },
        requesters::client::HTTPClient,
        types::query::SearchQueryParam,
    };

    pub fn generic_search_engine(url: &str) -> GenericSearchEngineModule {
        let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
        let extractor = RegexExtractor::default();
        let url = Url::parse(url);

        GenericSearchEngineModule {
            name: TEST_MODULE_NAME.to_string(),
            url: url.unwrap(),
            param: SearchQueryParam::from("q"),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn generic_api_integration(url: &str) -> GenericAPIIntegrationModule {
        let parse = |json: Value| {
            if let Some(subs) = json["subdomains"].as_array() {
                let filter = |item: &Value| Some(item.as_str()?.to_string());

                BTreeSet::from_iter(subs.iter().filter_map(filter))
            } else {
                BTreeSet::default()
            }
        };

        let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
        let extractor = JSONExtractor::new(Box::new(parse));
        let url = url.to_string();

        GenericAPIIntegrationModule {
            name: TEST_MODULE_NAME.to_string(),
            url: Box::new(move |_| url.clone()),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }
}
