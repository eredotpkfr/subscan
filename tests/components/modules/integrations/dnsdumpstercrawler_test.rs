use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
};
use subscan::{
    enums::{
        content::Content, dispatchers::SubscanModuleDispatcher, module::SubscanModuleStatus::Failed,
    },
    interfaces::module::SubscanModuleInterface,
    modules::integrations::dnsdumpstercrawler::DNSDumpsterCrawler,
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

    assert_eq!(result.subdomains, [].into());
    assert_eq!(result.status, Failed("not get token".into()));
}

#[tokio::test]
#[stubr::mock("module/integrations/dnsdumpstercrawler")]
async fn run_test_with_token() {
    let mut dnsdumpstercrawler = DNSDumpsterCrawler::dispatcher();

    funcs::wrap_module_url(
        &mut dnsdumpstercrawler,
        &stubr.path("/dnsdumpstercrawler-with-token"),
    );

    let results = dnsdumpstercrawler.run(TEST_DOMAIN).await;

    assert_eq!(results.subdomains, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
