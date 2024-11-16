use std::{collections::BTreeSet, env};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};
use serde_json::Value;
use subscan::{
    enums::content::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::dnsdumpsterapi::{DNSDumpsterAPI, DNSDUMPSTERAPI_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/dnsdumpsterapi.json")]
async fn run_test() {
    let mut dnsdumpsterapi = DNSDumpsterAPI::dispatcher();
    let env_name = dnsdumpsterapi.envs().await.apikey.name;

    env::set_var(&env_name, "dnsdumpsterapi-api-key");
    funcs::wrap_module_url(&mut dnsdumpsterapi, &stubr.path("/dnsdumpsterapi"));

    let result = dnsdumpsterapi.run(TEST_DOMAIN).await;

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = DNSDumpsterAPI::get_query_url(TEST_DOMAIN);
    let expected = format!("{DNSDUMPSTERAPI_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = DNSDumpsterAPI::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/dnsdumpsterapi.json")["response"]["jsonBody"].clone();

    let extracted = DNSDumpsterAPI::extract(json, TEST_DOMAIN);
    let not_extracted = DNSDumpsterAPI::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
