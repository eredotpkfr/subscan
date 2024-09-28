use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    mocks::wrap_url_with_mock_func,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::bufferover::{self, BUFFEROVER_MODULE_NAME, BUFFEROVER_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/bufferover.json")]
async fn bufferover_run_test() {
    let mut bufferover = bufferover::Bufferover::new();
    let (env_name, _) = bufferover.fetch_apikey().await;

    env::set_var(&env_name, "bufferover-api-key");

    bufferover.url = wrap_url_with_mock_func(stubr.path("/bufferover").as_str());

    let result = bufferover.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(bufferover.name().await, BUFFEROVER_MODULE_NAME);
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
    let url = bufferover::Bufferover::get_query_url(TEST_DOMAIN);
    let expected = format!("{BUFFEROVER_URL}/dns?q={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let content = "{\"Results\": [\"127.0.0.1,md5,,bar.foo.com\"]}";
    let json = serde_json::from_str(content).unwrap();

    let extracted = bufferover::Bufferover::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = bufferover::Bufferover::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
