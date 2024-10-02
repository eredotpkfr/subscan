use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
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
    let mut bufferover = bufferover::Bufferover::dispatcher();
    let (env_name, _) = bufferover.fetch_apikey().await;

    env::set_var(&env_name, "bufferover-api-key");
    mocks::wrap_module_dispatcher_url(&mut bufferover, &stubr.path("/bufferover"));

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
    let json = read_stub("module/integrations/bufferover.json")["response"]["jsonBody"].clone();
    let extracted = bufferover::Bufferover::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = bufferover::Bufferover::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(
        extracted,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string(),
        ]
        .into()
    );
    assert_eq!(not_extracted, BTreeSet::new());
}
