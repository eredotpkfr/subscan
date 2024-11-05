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
        fs,
        net::TcpListener,
        path::{Path, PathBuf},
    };

    pub fn stubs_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/stubs")
    }

    pub fn get_random_port() -> u16 {
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
}

pub mod stub {
    use super::funcs::{get_random_port, stubs_path};
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };
    use tempfile::{tempdir_in, TempDir};
    use tokio::sync::OnceCell;

    pub struct TempStubManager<'a> {
        pub temp: TempDir,
        pub stubs: PathBuf,
        pub templates: Vec<&'a str>,
        pub port: u16,
        pub init: OnceCell<()>,
    }

    impl<'a> From<(&'a str, Vec<&'a str>)> for TempStubManager<'a> {
        fn from(tuple: (&'a str, Vec<&'a str>)) -> Self {
            Self::new(tuple.0, tuple.1)
        }
    }

    impl<'a> TempStubManager<'a> {
        pub fn new(stubs: &'a str, templates: Vec<&'a str>) -> Self {
            let stubs = stubs_path().join(stubs);
            let temp = tempdir_in(stubs.clone()).unwrap();
            let port = get_random_port();
            let init = OnceCell::new();

            Self {
                temp,
                stubs,
                templates,
                port,
                init,
            }
        }

        pub async fn port(&self) -> u16 {
            self.init().await;
            self.port
        }

        pub async fn temp(&self) -> &str {
            self.init().await;
            self.temp.path().to_str().unwrap()
        }

        async fn init(&self) {
            self.init
                .get_or_init(|| async {
                    for entry in self.stubs.read_dir().unwrap().flatten() {
                        if entry.path().is_file() {
                            let name = entry.file_name();
                            let target_path = self.temp.path().join(&name);

                            if self.templates.contains(&name.to_str().unwrap()) {
                                self.fill_port(entry.path(), target_path).await;
                            } else {
                                fs::copy(entry.path(), target_path).unwrap();
                            }
                        }
                    }
                })
                .await;
        }

        async fn fill_port(&self, input: PathBuf, output: PathBuf) {
            let template = fs::read_to_string(input).unwrap();
            let filled_stub = template.replace("{{port}}", &self.port.to_string());
            let mut temp_stub = File::create(output).unwrap();

            temp_stub.write_all(filled_stub.as_bytes()).unwrap();
        }
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
            SubscanModuleDispatcher::ZoneTransfer(_) => {}
        }
    }
}
