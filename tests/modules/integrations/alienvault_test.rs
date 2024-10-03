use std::collections::BTreeSet;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::alienvault::{self, ALIENVAULT_MODULE_NAME, ALIENVAULT_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/alienvault.json")]
async fn alienvault_run_test() {
    let mut alienvault = alienvault::AlienVault::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut alienvault, &stubr.path("/alienvault"));

    let result = alienvault.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(alienvault.name().await, ALIENVAULT_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = alienvault::AlienVault::get_query_url(TEST_DOMAIN);
    let expected = format!("{ALIENVAULT_URL}/{TEST_DOMAIN}/passive_dns");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/alienvault.json")["response"]["jsonBody"].clone();

    let extracted = alienvault::AlienVault::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = alienvault::AlienVault::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
