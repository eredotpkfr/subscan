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
    modules::integrations::zoomeye::{ZoomEye, ZOOMEYE_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/zoomeye.json")]
async fn run_test() {
    let mut zoomeye = ZoomEye::dispatcher();
    let env_name = zoomeye.envs().await.apikey.name;

    env::set_var(&env_name, "zoomeye-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut zoomeye, &stubr.path("/zoomeye"));

    let result = zoomeye.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let params = &[("q", TEST_DOMAIN), ("s", "250"), ("type", "1")];
    let expected = Url::parse_with_params(ZOOMEYE_URL, params);
    let url = ZoomEye::get_query_url(TEST_DOMAIN);

    assert_eq!(url, expected.unwrap().to_string());
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();

    let mut next = ZoomEye::get_next_url(url.clone(), Content::Empty).unwrap();
    let mut expected = Url::parse(&format!("{TEST_URL}/?page=2")).unwrap();

    assert_eq!(next, expected);

    next = ZoomEye::get_next_url(next, Content::Empty).unwrap();
    expected = Url::parse(&format!("{TEST_URL}/?page=3")).unwrap();

    assert_eq!(next, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/zoomeye.json")["response"]["jsonBody"].clone();

    let extracted = ZoomEye::extract(json, TEST_DOMAIN);
    let not_extracted = ZoomEye::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
