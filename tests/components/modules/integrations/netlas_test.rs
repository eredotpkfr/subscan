use std::env;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils::read_stub,
};
use serde_json::Value;
use subscan::{
    error::{ModuleErrorKind::JSONExtract, SkipReason::AuthenticationNotProvided, SubscanError},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::netlas::Netlas,
};

#[tokio::test]
#[stubr::mock("module/integrations/netlas")]
async fn run_test() {
    let mut netlas = Netlas::dispatcher();
    let env_name = netlas.envs().await.apikey.name;

    env::set_var(&env_name, "netlas-api-key");
    funcs::wrap_module_url(&mut netlas, &stubr.uri());

    let results = netlas.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(results.subdomains, [TEST_BAR_SUBDOMAIN.to_string()].into());

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/integrations/netlas/netlas-no-count.json")]
async fn run_test_no_count() {
    let mut netlas = Netlas::dispatcher();

    funcs::wrap_module_url(&mut netlas, &stubr.uri());

    let results = netlas.run(TEST_DOMAIN).await;

    // Test no count response
    assert!(results.is_err());
    assert_eq!(results.err().unwrap(), AuthenticationNotProvided.into());
}

#[tokio::test]
async fn extract_test() {
    let stub = read_stub("module/integrations/netlas/netlas-domains-download.json");
    let json = stub["response"]["jsonBody"].clone();

    let extracted = Netlas::extract(json, TEST_DOMAIN);
    let not_extracted = Netlas::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(
        not_extracted.err().unwrap(),
        SubscanError::from(JSONExtract)
    );
}
