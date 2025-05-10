use std::{collections::BTreeSet, env};

use serde_json::Value;
use subscan::{
    error::ModuleErrorKind::JSONExtract, interfaces::module::SubscanModuleInterface,
    modules::integrations::netlas::Netlas, types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/netlas")]
async fn run_test() {
    let mut netlas = Netlas::dispatcher();
    let env_name = netlas.envs().await.apikey.name;

    env::set_var(&env_name, "netlas-api-key");
    funcs::wrap_module_url(&mut netlas, &stubr.uri());

    let (results, status) = utils::run_module(netlas, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/integrations/netlas/netlas-no-count.json")]
async fn run_test_no_count() {
    let mut netlas = Netlas::dispatcher();

    funcs::wrap_module_url(&mut netlas, &stubr.uri());

    let (results, status) = utils::run_module(netlas, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, "json parse error".into());
}

#[tokio::test]
async fn extract_test() {
    let stub = "module/integrations/netlas/netlas-domains-download.json";
    let json = utils::read_stub(stub)["response"]["jsonBody"].clone();

    let extracted = Netlas::extract(json, TEST_DOMAIN);
    let not_extracted = Netlas::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
