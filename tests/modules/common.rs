use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, thread};

pub mod constants {
    pub const TEST_URL: &str = "http://foo.com";
    pub const TEST_DOMAIN: &str = "foo.com";
    pub const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";
    pub const TEST_BAZ_SUBDOMAIN: &str = "baz.foo.com";
    pub const TEST_API_KEY: &str = "test-api-key";
}

pub mod funcs {
    pub fn md5_hex(target: String) -> String {
        format!("{:x}", md5::compute(target))
    }
}

pub mod mocks {
    use super::funcs::md5_hex;
    use super::*;
    use subscan::{
        enums::{APIAuthMethod, RequesterDispatcher},
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
        let thread_name = thread::current().name().unwrap().to_uppercase();

        GenericSearchEngineModule {
            name: md5_hex(thread_name),
            url: url.unwrap(),
            param: SearchQueryParam::from("q"),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn generic_api_integration(url: &str, auth: APIAuthMethod) -> GenericAPIIntegrationModule {
        let parse = |json: Value| {
            if let Some(subs) = json["subdomains"].as_array() {
                let filter = |item: &Value| Some(item.as_str()?.to_string());

                BTreeSet::from_iter(subs.iter().filter_map(filter))
            } else {
                BTreeSet::new()
            }
        };

        let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
        let extractor = JSONExtractor::new(Box::new(parse));
        let thread_name = thread::current().name().unwrap().to_uppercase();

        GenericAPIIntegrationModule {
            name: md5_hex(thread_name),
            url: wrap_url_with_mock_func(url),
            auth,
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn wrap_url_with_mock_func(url: &str) -> Box<dyn Fn(&str) -> String + Sync + Send> {
        let url: Url = url.parse().unwrap();

        Box::new(move |_| url.to_string().clone())
    }
}
