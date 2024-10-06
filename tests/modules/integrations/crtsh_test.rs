use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use std::collections::BTreeSet;
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::crtsh::{self, CRTSH_MODULE_NAME, CRTSH_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/crtsh.json")]
async fn crtsh_run_test() {
    let mut crtsh = crtsh::Crtsh::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut crtsh, &stubr.path("/crtsh"));

    let result = crtsh.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(crtsh.name().await, CRTSH_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = crtsh::Crtsh::get_query_url(TEST_DOMAIN);

    assert_eq!(url, format!("{CRTSH_URL}/?q={TEST_DOMAIN}&output=json"));
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/crtsh.json")["response"]["jsonBody"].clone();
    let extracted = crtsh::Crtsh::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = crtsh::Crtsh::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
