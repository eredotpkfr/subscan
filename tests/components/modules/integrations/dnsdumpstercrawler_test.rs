use subscan::{
    enums::{content::Content, dispatchers::SubscanModuleDispatcher},
    error::ModuleErrorKind::Custom,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::dnsdumpstercrawler::DNSDumpsterCrawler,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
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

    let result = dnsdumpstercrawler.run(TEST_DOMAIN).await;

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Custom("not get token".into()).into());
}

#[tokio::test]
#[stubr::mock("module/integrations/dnsdumpstercrawler")]
async fn run_test_with_token() {
    let mut dnsdumpstercrawler = DNSDumpsterCrawler::dispatcher();

    funcs::wrap_module_url(
        &mut dnsdumpstercrawler,
        &stubr.path("/dnsdumpstercrawler-with-token"),
    );

    let results = dnsdumpstercrawler.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(results.subdomains, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
