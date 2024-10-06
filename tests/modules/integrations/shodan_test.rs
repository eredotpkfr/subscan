use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::shodan::{self, SHODAN_MODULE_NAME, SHODAN_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/shodan.json")]
async fn shodan_run_test() {
    let mut shodan = shodan::Shodan::dispatcher();
    let (env_name, _) = shodan.fetch_apikey().await;

    env::set_var(&env_name, "shodan-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut shodan, &stubr.path("/shodan"));

    let result = shodan.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(shodan.name().await, SHODAN_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = shodan::Shodan::get_query_url(TEST_DOMAIN);
    let expected = format!("{SHODAN_URL}/dns/domain/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/shodan.json")["response"]["jsonBody"].clone();
    let extracted = shodan::Shodan::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = shodan::Shodan::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
