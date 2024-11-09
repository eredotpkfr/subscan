use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
};
use subscan::{
    enums::content::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::digitorus::{Digitorus, DIGITORUS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/digitorus.json")]
async fn run_test() {
    let mut digitorus = Digitorus::dispatcher();

    funcs::wrap_module_dispatcher_url_field(&mut digitorus, &stubr.path("/digitorus"));

    let result = digitorus.run(TEST_DOMAIN).await;

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
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
