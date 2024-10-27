use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    enums::{Content, SubscanModuleDispatcher},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::dnsdumpster::DnsDumpster,
};

#[tokio::test]
async fn get_csrf_token_test() {
    if let SubscanModuleDispatcher::DnsDumpster(module) = DnsDumpster::dispatcher() {
        let content = r#"<input type="hidden" name="csrfmiddlewaretoken" value="foo">"#;

        assert_eq!(module.get_csrf_token(Content::Empty).await, None);
        assert_eq!(module.get_csrf_token(content.into()).await.unwrap(), "foo");
    }
}

#[tokio::test]
#[stubr::mock("module/integrations/dnsdumpster")]
async fn run_test_no_token() {
    let mut dnsdumpster = DnsDumpster::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut dnsdumpster, &stubr.path("/dnsdumpster-no-token"));

    assert_eq!(dnsdumpster.run(TEST_DOMAIN).await.subdomains, [].into());
}

#[tokio::test]
#[stubr::mock("module/integrations/dnsdumpster")]
async fn run_test_with_token() {
    let mut dnsdumpster = DnsDumpster::dispatcher();

    mocks::wrap_module_dispatcher_url_field(
        &mut dnsdumpster,
        &stubr.path("/dnsdumpster-with-token"),
    );

    let results = dnsdumpster.run(TEST_DOMAIN).await;

    assert_eq!(results.subdomains, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
