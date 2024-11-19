use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::google::Google};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn run_test() {
    let mut google = Google::dispatcher();

    funcs::wrap_module_url(&mut google, &stubr.path("/search"));

    let result = google.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}
