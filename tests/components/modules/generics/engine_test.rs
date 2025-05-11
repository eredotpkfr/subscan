use std::{collections::BTreeSet, time::Duration};

use subscan::{
    error::ModuleErrorKind::{GetContent, RegexExtract},
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    types::{config::requester::RequesterConfig, result::status::SubscanModuleStatus},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::modules,
    utils,
};

#[tokio::test]
async fn attribute_test() {
    let module = modules::generic_search_engine(TEST_URL);
    let envs = module.envs().await;

    assert_eq!(module.name().await, module.name);

    assert!(envs.apikey.value.is_none());
    assert!(envs.credentials.username.value.is_none());
    assert!(envs.credentials.password.value.is_none());

    assert!(module.requester().await.is_some());
    assert!(module.extractor().await.is_some());
}

#[tokio::test]
#[stubr::mock("module/generics/search-engine.json")]
async fn run_success_test() {
    let module = modules::generic_search_engine(&stubr.path("/search"));

    let (results, status) = utils::run_module(module.into(), TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
#[stubr::mock("module/generics/search-engine.json")]
async fn run_fail_extract_test() {
    let module = modules::generic_search_engine(&stubr.path("/search"));

    let (results, status) = utils::run_module(module.into(), "{}}").await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, RegexExtract.into());
}

#[tokio::test]
#[stubr::mock("module/generics/search-engine-delayed.json")]
async fn run_fail_get_content_test() {
    let rconfig = RequesterConfig {
        timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let module = modules::generic_search_engine(&stubr.path("/search"));

    module.requester().await.unwrap().lock().await.configure(rconfig).await;

    let (results, status) = utils::run_module(module.into(), TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, GetContent.into());
}
