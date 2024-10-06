use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use std::collections::BTreeSet;
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::leakix::{self, LEAKIX_MODULE_NAME, LEAKIX_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/leakix.json")]
async fn run_test() {
    let mut leakix = leakix::Leakix::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut leakix, &stubr.path("/leakix"));

    let result = leakix.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(leakix.name().await, LEAKIX_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = leakix::Leakix::get_query_url(TEST_DOMAIN);
    let expected = format!("{LEAKIX_URL}/subdomains/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/leakix.json")["response"]["jsonBody"].clone();
    let extracted = leakix::Leakix::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = leakix::Leakix::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
