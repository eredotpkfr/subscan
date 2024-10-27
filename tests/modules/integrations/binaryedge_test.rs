use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    funcs::read_stub,
    mocks,
};
use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, env};
use subscan::{
    enums::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::binaryedge::{BinaryEdge, BINARYEDGE_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/binaryedge.json")]
async fn run_test() {
    let mut binaryedge = BinaryEdge::dispatcher();
    let env_name = binaryedge.envs().await.apikey.name;

    env::set_var(&env_name, "binaryedge-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut binaryedge, &stubr.path("/binaryedge"));

    let result = binaryedge.run(TEST_DOMAIN).await;

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = BinaryEdge::get_query_url(TEST_DOMAIN);
    let expected = format!("{BINARYEDGE_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();

    let mut next = BinaryEdge::get_next_url(url.clone(), Content::Empty).unwrap();
    let mut expected = Url::parse(&format!("{TEST_URL}/?page=2")).unwrap();

    assert_eq!(next, expected);

    next = BinaryEdge::get_next_url(next, Content::Empty).unwrap();
    expected = Url::parse(&format!("{TEST_URL}/?page=3")).unwrap();

    assert_eq!(next, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/binaryedge.json")["response"]["jsonBody"].clone();

    let extracted = BinaryEdge::extract(json, TEST_DOMAIN);
    let not_extracted = BinaryEdge::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
