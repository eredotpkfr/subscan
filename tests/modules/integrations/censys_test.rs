use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::censys::{self, CENSYS_MODULE_NAME, CENSYS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/censys.json")]
async fn censys_run_test() {
    let mut censys = censys::Censys::dispatcher();
    let (env_name, _) = censys.fetch_apikey().await;

    env::set_var(&env_name, "censys-api-key");
    mocks::wrap_module_dispatcher_url(&mut censys, &stubr.path("/censys"));

    let result = censys.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(censys.name().await, CENSYS_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = censys::Censys::get_query_url(TEST_DOMAIN);

    assert_eq!(url, format!("{CENSYS_URL}?q={TEST_DOMAIN}"));
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/censys.json")["response"]["jsonBody"].clone();
    let extracted = censys::Censys::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = censys::Censys::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
