use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, thread};

pub mod constants {
    pub const TEST_URL: &str = "http://foo.com";
    pub const TEST_DOMAIN: &str = "foo.com";
    pub const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";
    pub const TEST_API_KEY: &str = "test-api-key";
    pub const READ_ERROR: &str = "Cannot read file!";
}

pub mod funcs {
    use super::constants::READ_ERROR;
    use serde_json::Value;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn stubs_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/stubs")
    }

    pub fn md5_hex(target: String) -> String {
        format!("{:x}", md5::compute(target))
    }

    pub fn read_stub(path: &str) -> Value {
        let file_path = stubs_path().join(path);
        let content = fs::read_to_string(file_path).expect(READ_ERROR);

        serde_json::from_str(&content).unwrap()
    }
}

pub mod mocks {
    use super::funcs::md5_hex;
    use super::*;
    use subscan::{
        enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
        extractors::{json::JSONExtractor, regex::RegexExtractor},
        modules::generics::{
            engine::GenericSearchEngineModule, integration::GenericIntegrationModule,
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

    pub fn generic_integration(url: &str, auth: APIAuthMethod) -> GenericIntegrationModule {
        let parse = |json: Value, _domain: String| {
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

        GenericIntegrationModule {
            name: md5_hex(thread_name),
            url: wrap_url_with_mock_func(url),
            next: Box::new(|_, _| None),
            auth,
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    fn wrap_url_with_mock_func(url: &str) -> Box<dyn Fn(&str) -> String + Sync + Send> {
        let url: Url = url.parse().unwrap();

        Box::new(move |_| url.to_string().clone())
    }

    pub fn wrap_module_dispatcher_url_field(dispatcher: &mut SubscanModuleDispatcher, url: &str) {
        match dispatcher {
            SubscanModuleDispatcher::GenericSearchEngineModule(module) => {
                module.url = url.parse().unwrap()
            }
            SubscanModuleDispatcher::GenericIntegrationModule(module) => {
                module.url = wrap_url_with_mock_func(url)
            }
        }
    }
}
