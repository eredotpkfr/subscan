use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::chaos::{self, CHAOS_MODULE_NAME, CHAOS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/chaos.json")]
async fn chaos_run_test() {
    let mut chaos = chaos::Chaos::dispatcher();
    let (env_name, _) = chaos.fetch_apikey().await;

    env::set_var(&env_name, "chaos-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut chaos, &stubr.path("/chaos"));

    let result = chaos.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(chaos.name().await, CHAOS_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = chaos::Chaos::get_query_url(TEST_DOMAIN);

    assert_eq!(url, format!("{CHAOS_URL}/{TEST_DOMAIN}/subdomains"));
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/chaos.json")["response"]["jsonBody"].clone();
    let extracted = chaos::Chaos::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = chaos::Chaos::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
