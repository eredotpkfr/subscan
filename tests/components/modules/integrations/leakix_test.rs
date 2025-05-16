use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtract,
    modules::integrations::leakix::{Leakix, LEAKIX_URL},
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/leakix.json")]
async fn run_test() {
    let mut leakix = Leakix::dispatcher();

    funcs::wrap_module_url(&mut leakix, &stubr.path("/leakix"));

    let (results, status) = utils::run_module(leakix, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Leakix::get_query_url(TEST_DOMAIN);
    let expected = format!("{LEAKIX_URL}/subdomains/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Leakix::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let stub = "module/integrations/leakix.json";
    let json = utils::read_stub(stub)["response"]["jsonBody"].clone();

    let extracted = Leakix::extract(json, TEST_DOMAIN);
    let not_extracted = Leakix::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
