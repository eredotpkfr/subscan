use core::time::Duration;
use std::collections::BTreeSet;

use serde_json::{json, Value};
use subscan::{
    enums::dispatchers::SubscanModuleDispatcher,
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    modules::integrations::commoncrawl::CommonCrawl,
    types::{config::requester::RequesterConfig, result::status::SubscanModuleStatus},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    stub::StubTemplateManager,
    utils,
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
    let manager: StubTemplateManager = (stubs, templates).into();

    let config = stubr::Config {
        port: Some(manager.port().await),
        ..Default::default()
    };

    let stubr = stubr::Stubr::start_with(manager.temp().await, config).await;
    let mut commoncrawl = CommonCrawl::dispatcher();

    funcs::wrap_module_url(&mut commoncrawl, &stubr.path("/commoncrawl/index"));

    if let SubscanModuleDispatcher::CommonCrawl(ref mut module) = commoncrawl {
        let requester = &mut *module.requester().await.unwrap().lock().await;

        // Set timeout for testing did not get response case
        let config = RequesterConfig {
            timeout: Duration::from_millis(10),
            ..Default::default()
        };

        requester.configure(config).await;
    };

    let (results, status) = utils::run_module(commoncrawl, TEST_DOMAIN).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert_eq!(results, expected);
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
#[stubr::mock("module/integrations/commoncrawl/commoncrawl-index-no-data.json")]
async fn run_failed_test() {
    let mut commoncrawl = CommonCrawl::dispatcher();

    funcs::wrap_module_url(&mut commoncrawl, &stubr.path("/commoncrawl/index-no-data"));

    let (results, status) = utils::run_module(commoncrawl, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, "not get cdx URLs".into());
}
