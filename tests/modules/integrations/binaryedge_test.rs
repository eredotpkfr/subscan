use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::binaryedge::{self, BINARYEDGE_MODULE_NAME, BINARYEDGE_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/binaryedge.json")]
async fn binaryedge_run_test() {
    let mut binaryedge = binaryedge::Binaryedge::dispatcher();
    let (env_name, _) = binaryedge.fetch_apikey().await;

    env::set_var(&env_name, "binaryedge-api-key");
    mocks::wrap_module_dispatcher_url(&mut binaryedge, &stubr.path("/binaryedge"));

    let result = binaryedge.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(binaryedge.name().await, BINARYEDGE_MODULE_NAME);
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
    let url = binaryedge::Binaryedge::get_query_url(TEST_DOMAIN);
    let expected = format!("{BINARYEDGE_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/binaryedge.json")["response"]["jsonBody"].clone();

    let extracted = binaryedge::Binaryedge::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = binaryedge::Binaryedge::extract(Value::Null, TEST_DOMAIN.to_string());

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
