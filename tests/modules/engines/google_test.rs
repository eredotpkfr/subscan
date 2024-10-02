use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::engines::google::{self, GOOGLE_MODULE_NAME},
};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn google_run_test() {
    let mut google = google::Google::dispatcher();

    mocks::wrap_module_dispatcher_url(&mut google, &stubr.path("/search"));

    let result = google.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(google.name().await, GOOGLE_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
