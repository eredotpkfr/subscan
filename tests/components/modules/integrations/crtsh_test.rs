use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtract,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::crtsh::{Crtsh, CRTSH_URL},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};

#[tokio::test]
#[stubr::mock("module/integrations/crtsh.json")]
async fn run_test() {
    let mut crtsh = Crtsh::dispatcher();

    funcs::wrap_module_url(&mut crtsh, &stubr.path("/crtsh"));

    let result = crtsh.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Crtsh::get_query_url(TEST_DOMAIN);
    let expected = format!("{CRTSH_URL}/?q={TEST_DOMAIN}&output=json");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Crtsh::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/crtsh.json")["response"]["jsonBody"].clone();
    let extracted = Crtsh::extract(json, TEST_DOMAIN);
    let not_extracted = Crtsh::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
