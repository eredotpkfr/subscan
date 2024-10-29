use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::google::Google};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn run_test() {
    let mut google = Google::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut google, &stubr.path("/search"));

    let result = google.run(TEST_DOMAIN).await;

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}
