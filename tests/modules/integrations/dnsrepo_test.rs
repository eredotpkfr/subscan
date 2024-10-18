use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mocks,
};
use subscan::{
    enums::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::dnsrepo::{DnsRepo, DNSREPO_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/dnsrepo.json")]
async fn run_test() {
    let mut dnsrepo = DnsRepo::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut dnsrepo, &stubr.path("/dnsrepo"));

    let result = dnsrepo.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
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
