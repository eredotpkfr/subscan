use subscan::{modules::engines::google::Google, types::result::status::SubscanModuleStatus};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn run_test() {
    let mut google = Google::dispatcher();

    funcs::wrap_module_url(&mut google, &stubr.path("/search"));

    let (results, status) = utils::run_module(google, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}
