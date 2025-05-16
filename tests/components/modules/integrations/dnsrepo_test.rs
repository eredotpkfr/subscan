use subscan::{
    enums::content::Content,
    modules::integrations::dnsrepo::{DnsRepo, DNSREPO_URL},
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/dnsrepo.json")]
async fn run_test() {
    let mut dnsrepo = DnsRepo::dispatcher();

    funcs::wrap_module_url(&mut dnsrepo, &stubr.path("/dnsrepo"));

    let (results, status) = utils::run_module(dnsrepo, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = DnsRepo::get_query_url(TEST_DOMAIN);
    let expected = format!("{DNSREPO_URL}/?search={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = DnsRepo::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}
