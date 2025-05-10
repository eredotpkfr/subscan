use reqwest::Url;
use subscan::{
    enums::dispatchers::SubscanModuleDispatcher,
    modules::integrations::netcraft::{Netcraft, NETCRAFT_URL},
    requesters::client::HTTPClient,
    types::result::status::SubscanModuleStatus,
};
use tokio::sync::Mutex;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/netcraft.json")]
async fn run_test() {
    let mut netcraft = Netcraft::dispatcher();
    let new_requester = HTTPClient::default();

    funcs::wrap_module_url(&mut netcraft, &stubr.path("/netcraft"));

    if let SubscanModuleDispatcher::GenericIntegrationModule(ref mut netcraft) = netcraft {
        netcraft.components.requester = Mutex::new(new_requester.into());
    }

    let (results, status) = utils::run_module(netcraft, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
async fn get_query_url_test() {
    let params = &[("restriction", "site+ends+with"), ("host", TEST_DOMAIN)];

    let expected = Url::parse_with_params(NETCRAFT_URL, params).unwrap();
    let url = Netcraft::get_query_url(TEST_DOMAIN);

    assert_eq!(url, expected.to_string());
}

#[tokio::test]
async fn get_next_url_test() {
    let html = "<table></table><p><a href=\"/page-2\"></a></p>";

    let url = TEST_URL.parse().unwrap();
    let next = Netcraft::get_next_url(url, html.into());
    let expected = format!("{NETCRAFT_URL}/page-2");

    assert_eq!(next.unwrap().to_string(), expected);
}

#[tokio::test]
async fn get_next_url_fail_test() {
    let html = "<table></table><p><a></a></p>";

    let url = TEST_URL.parse().unwrap();
    let next = Netcraft::get_next_url(url, html.into());

    assert!(next.is_none());
}
