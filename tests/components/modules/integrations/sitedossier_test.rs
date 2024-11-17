use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
};
use subscan::{
    enums::content::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::sitedossier::{Sitedossier, SITEDOSSIER_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/sitedossier.json")]
async fn run_test() {
    let mut sitedossier = Sitedossier::dispatcher();

    funcs::wrap_module_url(&mut sitedossier, &stubr.path("/sitedossier"));

    let result = sitedossier.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
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
