use subscan::{
    enums::content::Content,
    modules::integrations::digitorus::{Digitorus, DIGITORUS_URL},
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/digitorus.json")]
async fn run_test() {
    let mut digitorus = Digitorus::dispatcher();

    funcs::wrap_module_url(&mut digitorus, &stubr.path("/digitorus"));

    let (results, status) = utils::run_module(digitorus, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Digitorus::get_query_url(TEST_DOMAIN);
    let expected = format!("{DIGITORUS_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Digitorus::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}
