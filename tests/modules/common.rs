use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, thread};

pub mod constants {
    pub const TEST_URL: &str = "http://foo.com";
    pub const TEST_DOMAIN: &str = "foo.com";
    pub const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";
    pub const TEST_BAZ_SUBDOMAIN: &str = "baz.foo.com";
    pub const TEST_API_KEY: &str = "test-api-key";
    pub const READ_ERROR: &str = "Cannot read file!";
}

pub mod funcs {
    use super::constants::READ_ERROR;
    use serde_json::Value;
    use std::{
        fs::{self, File},
        io::Write,
        net::TcpListener,
        path::{Path, PathBuf},
    };

    fn stubs_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/stubs")
    }

    fn get_random_port() -> u16 {
        TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    pub fn md5_hex(target: String) -> String {
        format!("{:x}", md5::compute(target))
    }

    pub fn read_stub(path: &str) -> Value {
        let file_path = stubs_path().join(path);
        let content = fs::read_to_string(file_path).expect(READ_ERROR);

        serde_json::from_str(&content).unwrap()
    }

    pub fn create_tmp_stubs_with_port(stubs: &str, templates: Vec<&str>) -> (PathBuf, u16) {
        let stubs_path = stubs_path().join(stubs);
        let tmp_path = stubs_path.join("tmp");
        let port = get_random_port();

        fs::create_dir_all(tmp_path.clone()).unwrap();

        for dir in stubs_path.read_dir().unwrap() {
            if let Ok(stub) = dir {
                if stub.path().is_file() {
                    let file_name = stub.file_name();

                    if templates.contains(&file_name.to_str().unwrap()) {
                        let template = fs::read_to_string(stub.path()).unwrap();
                        let filled_stub = template.replace("{{port}}", &port.to_string());
                        let mut tmp_stub = File::create(tmp_path.join(file_name)).unwrap();

                        tmp_stub.write_all(filled_stub.as_bytes()).unwrap();
                    } else {
                        fs::copy(stub.path(), tmp_path.join(file_name)).unwrap();
                    }
                }
            }
        }

        (tmp_path, port)
    }
}

pub mod mocks {
    use super::funcs::md5_hex;
    use super::*;
    use subscan::{
        enums::{
            auth::AuthenticationMethod,
            dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
        },
        extractors::{json::JSONExtractor, regex::RegexExtractor},
        modules::generics::{
            engine::GenericSearchEngineModule, integration::GenericIntegrationModule,
        },
        requesters::client::HTTPClient,
        types::{
            core::SubscanModuleCoreComponents, func::GenericIntegrationCoreFuncs,
            query::SearchQueryParam,
        },
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
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        }
    }

    pub fn generic_integration(url: &str, auth: AuthenticationMethod) -> GenericIntegrationModule {
        let parse = |json: Value, _domain: &str| {
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
            auth,
            funcs: GenericIntegrationCoreFuncs {
                url: wrap_url_with_mock_func(url),
                next: Box::new(|_, _| None),
            },
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
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
                module.funcs.url = wrap_url_with_mock_func(url)
            }
            SubscanModuleDispatcher::CommonCrawl(module) => module.url = url.parse().unwrap(),
            SubscanModuleDispatcher::DnsDumpster(module) => module.url = url.parse().unwrap(),
            SubscanModuleDispatcher::GitHub(module) => module.url = url.parse().unwrap(),
            SubscanModuleDispatcher::Netlas(module) => module.url = url.parse().unwrap(),
            SubscanModuleDispatcher::WaybackArchive(module) => module.url = url.parse().unwrap(),
        }
    }
}
