use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    funcs::read_stub,
    mocks,
};
use reqwest::Url;
use serde_json::{json, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::censys::{Censys, CENSYS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/censys.json")]
async fn run_test() {
    let mut censys = Censys::dispatcher();
    let env_name = censys.envs().await.apikey.name;

    env::set_var(&env_name, "censys-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut censys, &stubr.path("/censys"));

    let result = censys.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Censys::get_query_url(TEST_DOMAIN);
    let expected = format!("{CENSYS_URL}?q={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();
    let json = json!({"result": {"links": {"next": "foo"}}});
    let expected = Url::parse(&format!("{TEST_URL}/?cursor=foo")).unwrap();

    let mut next = Censys::get_next_url(url.clone(), Value::Null);

    assert!(next.is_none());

    next = Censys::get_next_url(url, json);

    assert_eq!(next.unwrap(), expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/censys.json")["response"]["jsonBody"].clone();
    let extracted = Censys::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = Censys::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
