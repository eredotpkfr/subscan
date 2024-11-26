use std::collections::BTreeSet;

use async_trait::async_trait;
use subscan::{
    enums::dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher},
    extractors::regex::RegexExtractor,
    interfaces::module::SubscanModuleInterface,
    requesters::client::HTTPClient,
    types::{
        core::{Result, Subdomain},
        result::{module::SubscanModuleResult, status::SubscanModuleStatus::Finished},
    },
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

    async fn run(&mut self, _domain: &str) -> Result<SubscanModuleResult> {
        let mut result: SubscanModuleResult = self.name().await.into();

        let subdomains = BTreeSet::from_iter([
            Subdomain::from("bar.foo.com"),
            Subdomain::from("baz.foo.com"),
        ]);

        result.extend(subdomains);

        Ok(result.with_finished().await)
    }
}

#[tokio::main]
async fn main() {
    let requester: RequesterDispatcher = HTTPClient::default().into();
    let extracator: RegexExtractor = RegexExtractor::default();

    let mut module = CustomModule {
        requester: requester.into(),
        extractor: extracator.into(),
    };

    assert!(module.requester().await.is_some());
    assert!(module.extractor().await.is_some());

    assert_eq!(module.name().await, "name");

    let result = module.run("foo.com").await.unwrap();
    let expected = BTreeSet::from_iter([
        Subdomain::from("bar.foo.com"),
        Subdomain::from("baz.foo.com"),
    ]);

    assert_eq!(result.subdomains, expected);
    assert_eq!(result.status, Finished);
}
