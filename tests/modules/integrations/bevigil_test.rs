use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    mocks::wrap_url_with_mock_func,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::bevigil::{self, BEVIGIL_MODULE_NAME, BEVIGIL_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/bevigil.json")]
async fn bevigil_run_test() {
    let mut bevigil = bevigil::Bevigil::new();
    let (env_name, _) = bevigil.fetch_apikey().await;

    env::set_var(&env_name, "bevigil-api-key");

    bevigil.url = wrap_url_with_mock_func(stubr.path("/bevigil").as_str());

    let result = bevigil.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(bevigil.name().await, BEVIGIL_MODULE_NAME);
    assert_eq!(
        result,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string(),
        ]
        .into()
    );

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = bevigil::Bevigil::get_query_url(TEST_DOMAIN);
    let expected = format!("{BEVIGIL_URL}/{TEST_DOMAIN}/subdomains");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let content = "{\"subdomains\": [\"bar.foo.com\"]}";
    let json = serde_json::from_str(content).unwrap();

    let extracted = bevigil::Bevigil::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = bevigil::Bevigil::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
