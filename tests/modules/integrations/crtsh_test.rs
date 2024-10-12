use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    funcs::read_stub,
    mocks,
};
use serde_json::Value;
use std::collections::BTreeSet;
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::crtsh::{Crtsh, CRTSH_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/crtsh.json")]
async fn run_test() {
    let mut crtsh = Crtsh::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut crtsh, &stubr.path("/crtsh"));

    let result = crtsh.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Crtsh::get_query_url(TEST_DOMAIN);
    let expected = format!("{CRTSH_URL}/?q={TEST_DOMAIN}&output=json");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Crtsh::get_next_url(url, Value::Null);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/crtsh.json")["response"]["jsonBody"].clone();
    let extracted = Crtsh::extract(json, TEST_DOMAIN);
    let not_extracted = Crtsh::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
