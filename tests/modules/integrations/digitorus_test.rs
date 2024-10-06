use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::digitorus::{self, DIGITORUS_MODULE_NAME, DIGITORUS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/digitorus.json")]
async fn digitorus_run_test() {
    let mut digitorus = digitorus::Digitorus::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut digitorus, &stubr.path("/digitorus"));

    let result = digitorus.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(digitorus.name().await, DIGITORUS_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = digitorus::Digitorus::get_query_url(TEST_DOMAIN);
    let expected = format!("{DIGITORUS_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}
