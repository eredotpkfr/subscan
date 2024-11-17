use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};
use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::{ModuleErrorKind::JSONExtractError, SubscanError},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::anubis::{Anubis, ANUBIS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/anubis.json")]
async fn run_test() {
    let mut anubis = Anubis::dispatcher();

    funcs::wrap_module_url(&mut anubis, &stubr.path("/anubis"));

    let result = anubis.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Anubis::get_query_url(TEST_DOMAIN);
    let expected = format!("{ANUBIS_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Anubis::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/anubis.json")["response"]["jsonBody"].clone();

    let extracted = Anubis::extract(json, TEST_DOMAIN);
    let not_extracted = Anubis::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(
        not_extracted.err().unwrap(),
        SubscanError::from(JSONExtractError)
    );
}
