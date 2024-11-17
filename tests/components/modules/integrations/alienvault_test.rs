use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};
use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtractError,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::alienvault::{AlienVault, ALIENVAULT_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/alienvault.json")]
async fn run_test() {
    let mut alienvault = AlienVault::dispatcher();

    funcs::wrap_module_url(&mut alienvault, &stubr.path("/alienvault"));

    let result = alienvault.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = AlienVault::get_query_url(TEST_DOMAIN);
    let expected = format!("{ALIENVAULT_URL}/{TEST_DOMAIN}/passive_dns");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = AlienVault::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/alienvault.json")["response"]["jsonBody"].clone();

    let extracted = AlienVault::extract(json, TEST_DOMAIN);
    let not_extracted = AlienVault::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtractError.into());
}
