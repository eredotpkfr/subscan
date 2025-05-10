use std::collections::BTreeSet;

use subscan::{
    enums::{content::Content, dispatchers::SubscanModuleDispatcher},
    modules::integrations::dnsdumpstercrawler::DNSDumpsterCrawler,
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
async fn get_auth_token_test() {
    if let SubscanModuleDispatcher::DNSDumpsterCrawler(module) = DNSDumpsterCrawler::dispatcher() {
        let content = r#"Authorization": "foo"#;

        assert_eq!(module.get_auth_token(Content::Empty).await, None);
        assert_eq!(module.get_auth_token(content.into()).await.unwrap(), "foo");
    }
}

#[tokio::test]
#[stubr::mock("module/integrations/dnsdumpstercrawler")]
async fn run_test_no_token() {
    let mut dnsdumpstercrawler = DNSDumpsterCrawler::dispatcher();

    funcs::wrap_module_url(
        &mut dnsdumpstercrawler,
        &stubr.path("/dnsdumpstercrawler-no-token"),
    );

    let (results, status) = utils::run_module(dnsdumpstercrawler, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, "not get token".into());
}

#[tokio::test]
#[stubr::mock("module/integrations/dnsdumpstercrawler")]
async fn run_test_with_token() {
    let mut dnsdumpstercrawler = DNSDumpsterCrawler::dispatcher();

    funcs::wrap_module_url(
        &mut dnsdumpstercrawler,
        &stubr.path("/dnsdumpstercrawler-with-token"),
    );

    let (results, status) = utils::run_module(dnsdumpstercrawler, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}
