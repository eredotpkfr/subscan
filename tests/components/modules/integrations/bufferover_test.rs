use std::env;

use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::{ModuleErrorKind::JSONExtract, SubscanError},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::bufferover::{BufferOver, BUFFEROVER_URL},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};

#[tokio::test]
#[stubr::mock("module/integrations/bufferover.json")]
async fn run_test() {
    let mut bufferover = BufferOver::dispatcher();
    let env_name = bufferover.envs().await.apikey.name;

    env::set_var(&env_name, "bufferover-api-key");
    funcs::wrap_module_url(&mut bufferover, &stubr.path("/bufferover"));

    let result = bufferover.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = BufferOver::get_query_url(TEST_DOMAIN);
    let expected = format!("{BUFFEROVER_URL}/dns?q={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = BufferOver::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/bufferover.json")["response"]["jsonBody"].clone();
    let extracted = BufferOver::extract(json, TEST_DOMAIN);
    let not_extracted = BufferOver::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(
        not_extracted.err().unwrap(),
        SubscanError::from(JSONExtract)
    );
}
