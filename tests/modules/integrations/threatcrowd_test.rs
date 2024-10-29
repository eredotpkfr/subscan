use std::collections::BTreeSet;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    funcs::read_stub,
    mocks,
};
use serde_json::Value;
use subscan::{
    enums::content::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::threatcrowd::{ThreatCrowd, THREATCROWD_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/threatcrowd.json")]
async fn run_test() {
    let mut threatcrowd = ThreatCrowd::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut threatcrowd, &stubr.path("/threatcrowd"));

    let result = threatcrowd.run(TEST_DOMAIN).await;

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = ThreatCrowd::get_query_url(TEST_DOMAIN);
    let expected = format!("{THREATCROWD_URL}/?domain={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = ThreatCrowd::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/threatcrowd.json")["response"]["jsonBody"].clone();

    let extracted = ThreatCrowd::extract(json, TEST_DOMAIN);
    let not_extracted = ThreatCrowd::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
