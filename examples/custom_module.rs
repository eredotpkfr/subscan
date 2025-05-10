use std::collections::BTreeSet;

use async_trait::async_trait;
use flume::Sender;
use subscan::{
    enums::{
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher},
        result::{OptionalSubscanModuleResult, SubscanModuleResult},
    },
    extractors::regex::RegexExtractor,
    interfaces::module::SubscanModuleInterface,
    requesters::client::HTTPClient,
    types::{core::Subdomain, result::item::SubscanModuleResultItem},
};
use tokio::sync::Mutex;

pub struct CustomModule {
    pub requester: Mutex<RequesterDispatcher>,
    pub extractor: SubdomainExtractorDispatcher,
}

#[async_trait]
impl SubscanModuleInterface for CustomModule {
    async fn name(&self) -> &str {
        &"name"
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        Some(&self.requester)
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        Some(&self.extractor)
    }

    async fn run(&mut self, _domain: &str, results: Sender<OptionalSubscanModuleResult>) {
        let subdomains = BTreeSet::from_iter([Subdomain::from("bar.foo.com")]);

        for subdomain in &subdomains {
            results.send((self.name().await, subdomain).into()).unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let requester: RequesterDispatcher = HTTPClient::default().into();
    let extracator: RegexExtractor = RegexExtractor::default();

    let (tx, rx) = flume::unbounded::<OptionalSubscanModuleResult>();

    let mut module = CustomModule {
        requester: requester.into(),
        extractor: extracator.into(),
    };

    assert!(module.requester().await.is_some());
    assert!(module.extractor().await.is_some());

    assert_eq!(module.name().await, "name");

    module.run("foo.com", tx).await;

    let result = rx.recv().unwrap();
    let item = SubscanModuleResultItem {
        module: "name".into(),
        subdomain: Subdomain::from("bar.foo.com"),
    };
    let expected = &SubscanModuleResult::SubscanModuleResultItem(item);

    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap(), expected);
}
