use std::env;

use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtract,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::securitytrails::{SecurityTrails, SECURITYTRAILS_URL},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};

#[tokio::test]
#[stubr::mock("module/integrations/securitytrails.json")]
async fn run_test() {
    let mut securitytrails = SecurityTrails::dispatcher();
    let env_name = securitytrails.envs().await.apikey.name;

    env::set_var(&env_name, "securitytrails-api-key");
    funcs::wrap_module_url(&mut securitytrails, &stubr.path("/securitytrails"));

    let result = securitytrails.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = SecurityTrails::get_query_url(TEST_DOMAIN);
    let expected = format!("{SECURITYTRAILS_URL}/{TEST_DOMAIN}/subdomains");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = SecurityTrails::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/securitytrails.json")["response"]["jsonBody"].clone();

    let extracted = SecurityTrails::extract(json, TEST_DOMAIN);
    let not_extracted = SecurityTrails::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
