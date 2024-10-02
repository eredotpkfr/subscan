use std::collections::BTreeSet;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::anubis::{self, ANUBIS_MODULE_NAME, ANUBIS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/anubis.json")]
async fn anubis_run_test() {
    let mut anubis = anubis::Anubis::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut anubis, &stubr.path("/anubis"));

    let result = anubis.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(anubis.name().await, ANUBIS_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = anubis::Anubis::get_query_url(TEST_DOMAIN);
    let expected = format!("{ANUBIS_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/anubis.json")["response"]["jsonBody"].clone();

    let extracted = anubis::Anubis::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = anubis::Anubis::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
