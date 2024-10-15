use std::{collections::BTreeSet, env};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::Value;
use subscan::{interfaces::module::SubscanModuleInterface, modules::integrations::netlas::Netlas};

#[tokio::test]
#[stubr::mock("module/integrations/netlas")]
async fn run_test() {
    let mut netlas = Netlas::dispatcher();
    let env_name = netlas.envs().await.apikey.name;

    env::set_var(&env_name, "netlas-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut netlas, &stubr.uri());

    assert_eq!(
        netlas.run(TEST_DOMAIN).await,
        [TEST_BAR_SUBDOMAIN.to_string()].into()
    );

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/integrations/netlas/netlas-no-count.json")]
async fn run_test_no_count() {
    let mut netlas = Netlas::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut netlas, &stubr.uri());

    // Test no count response
    assert_eq!(netlas.run(TEST_DOMAIN).await, [].into());
}

#[tokio::test]
async fn extract_test() {
    let stub = read_stub("module/integrations/netlas/netlas-domains-download.json");
    let json = stub["response"]["jsonBody"].clone();

    let extracted = Netlas::extract(json, TEST_DOMAIN);
    let not_extracted = Netlas::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
