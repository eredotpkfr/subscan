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
    modules::integrations::whoisxmlapi::{WhoisXMLAPI, WHOISXMLAPI_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/whoisxmlapi.json")]
async fn run_test() {
    let mut whoisxmlapi = WhoisXMLAPI::dispatcher();
    let env_name = whoisxmlapi.envs().await.apikey.name;

    env::set_var(&env_name, "whoisxmlapi-api-key");
    funcs::wrap_module_url(&mut whoisxmlapi, &stubr.path("/whoisxmlapi"));

    let result = whoisxmlapi.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = WhoisXMLAPI::get_query_url(TEST_DOMAIN);
    let expected = format!("{WHOISXMLAPI_URL}/?domainName={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = WhoisXMLAPI::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/whoisxmlapi.json")["response"]["jsonBody"].clone();
    let extracted = WhoisXMLAPI::extract(json, TEST_DOMAIN);
    let not_extracted = WhoisXMLAPI::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(
        not_extracted.err().unwrap(),
        SubscanError::from(JSONExtractError)
    );
}
