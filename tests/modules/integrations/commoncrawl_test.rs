use core::time::Duration;
use std::collections::BTreeSet;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mocks,
    stub::TmpStubManager,
};
use serde_json::{json, Value};
use subscan::{
    enums::dispatchers::SubscanModuleDispatcher,
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    modules::integrations::commoncrawl::CommonCrawl,
    types::config::RequesterConfig,
};

#[tokio::test]
async fn extract_cdx_urls_test() {
    if let SubscanModuleDispatcher::CommonCrawl(module) = CommonCrawl::dispatcher() {
        let json = json!([
            {"no-cdx-api-field": TEST_URL},
            {"no-id-field": TEST_URL},
            {"id": "2024", "cdx-api": TEST_URL},
            {"id": "2024", "cdx-api": TEST_URL},
            {"id": "2023", "cdx-api": "https://foo.bar"},
        ]);
        let expected = [TEST_URL.to_string()];

        assert_eq!(module.extract_cdx_urls(Value::Null, "2024"), None);
        assert_eq!(
            module.extract_cdx_urls(json, "2024").unwrap(),
            expected.into()
        );
    }
}

#[tokio::test]
async fn run_test() {
    let stubs = "module/integrations/commoncrawl";
    let templates = vec!["commoncrawl-index-template.json"];
    let manager: TmpStubManager = (stubs, templates).into();

    let config = stubr::Config {
        port: Some(manager.port().await),
        ..Default::default()
    };

    let stubr = stubr::Stubr::start_with(manager.tmp().await, config).await;
    let mut commoncrawl = CommonCrawl::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut commoncrawl, &stubr.path("/commoncrawl/index"));

    if let SubscanModuleDispatcher::CommonCrawl(ref mut module) = commoncrawl {
        let requester = &mut *module.requester().await.unwrap().lock().await;

        // Set timeout for testing did not get response case
        let config = RequesterConfig {
            timeout: Duration::from_millis(10),
            ..Default::default()
        };

        requester.configure(config).await;
    };

    let results = commoncrawl.run(TEST_DOMAIN).await;
    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert_eq!(results.subdomains, expected);
}
