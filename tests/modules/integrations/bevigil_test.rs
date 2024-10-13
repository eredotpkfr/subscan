use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    funcs::read_stub,
    mocks,
};
use serde_json::Value;
use std::{collections::BTreeSet, env};
use subscan::{
    enums::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::bevigil::{Bevigil, BEVIGIL_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/bevigil.json")]
async fn run_test() {
    let mut bevigil = Bevigil::dispatcher();
    let env_name = bevigil.envs().await.apikey.name;

    env::set_var(&env_name, "bevigil-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut bevigil, &stubr.path("/bevigil"));

    let result = bevigil.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Bevigil::get_query_url(TEST_DOMAIN);
    let expected = format!("{BEVIGIL_URL}/{TEST_DOMAIN}/subdomains");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Bevigil::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/bevigil.json")["response"]["jsonBody"].clone();

    let extracted = Bevigil::extract(json, TEST_DOMAIN);
    let not_extracted = Bevigil::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
