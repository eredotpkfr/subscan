use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};
use serde_json::Value;
use std::env;
use subscan::{
    enums::content::Content,
    error::{ModuleErrorKind::JSONExtractError, SubscanError},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::chaos::{Chaos, CHAOS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/chaos.json")]
async fn run_test() {
    let mut chaos = Chaos::dispatcher();
    let env_name = chaos.envs().await.apikey.name;

    env::set_var(&env_name, "chaos-api-key");
    funcs::wrap_module_url(&mut chaos, &stubr.path("/chaos"));

    let result = chaos.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Chaos::get_query_url(TEST_DOMAIN);
    let expected = format!("{CHAOS_URL}/{TEST_DOMAIN}/subdomains");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Chaos::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/chaos.json")["response"]["jsonBody"].clone();
    let extracted = Chaos::extract(json, TEST_DOMAIN);
    let not_extracted = Chaos::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(
        not_extracted.err().unwrap(),
        SubscanError::from(JSONExtractError)
    );
}
