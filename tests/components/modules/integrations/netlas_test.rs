use std::{collections::BTreeSet, env, time::Duration};

use serde_json::Value;
use subscan::{
    error::ModuleErrorKind::{GetContent, JSONExtract},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::netlas::Netlas,
    types::result::status::{SkipReason::AuthenticationNotProvided, SubscanModuleStatus},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/netlas/with-count")]
async fn run_success_test() {
    let mut netlas = Netlas::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut netlas, utils::current_thread_hex());

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
async fn run_no_count_test() {
    let mut netlas = Netlas::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut netlas, utils::current_thread_hex());

    let env_name = netlas.envs().await.apikey.name;

    env::set_var(&env_name, "netlas-api-key");
    funcs::wrap_module_url(&mut netlas, &stubr.uri());

    let (results, status) = utils::run_module(netlas, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, "json parse error".into());

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/integrations/netlas/netlas-delayed.json")]
async fn run_timeout_test() {
    let mut netlas = Netlas::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut netlas, utils::current_thread_hex());

    let env_name = netlas.envs().await.apikey.name;

    env::set_var(&env_name, "netlas-api-key");
    funcs::wrap_module_url(&mut netlas, &stubr.uri());
    funcs::set_requester_timeout(&mut netlas, Duration::from_millis(500)).await;

    let (results, status) = utils::run_module(netlas, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, GetContent.into());

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/integrations/netlas/netlas-no-count.json")]
async fn run_no_auth_test() {
    let mut netlas = Netlas::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut netlas, utils::current_thread_hex());
    funcs::wrap_module_url(&mut netlas, &stubr.uri());

    let (results, status) = utils::run_module(netlas, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, AuthenticationNotProvided.into());
}

#[tokio::test]
async fn extract_test() {
    let stub = "module/integrations/netlas/with-count/netlas-domains-download.json";
    let json = utils::read_stub(stub)["response"]["jsonBody"].clone();

    let extracted = Netlas::extract(json, TEST_DOMAIN);
    let not_extracted = Netlas::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
