use subscan::{
    enums::content::Content,
    modules::integrations::sitedossier::{Sitedossier, SITEDOSSIER_URL},
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/sitedossier.json")]
async fn run_test() {
    let mut sitedossier = Sitedossier::dispatcher();

    funcs::wrap_module_url(&mut sitedossier, &stubr.path("/sitedossier"));

    let (results, status) = utils::run_module(sitedossier, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Sitedossier::get_query_url(TEST_DOMAIN);
    let expected = format!("{SITEDOSSIER_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Sitedossier::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}
