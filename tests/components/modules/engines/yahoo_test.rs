use subscan::{modules::engines::yahoo::Yahoo, types::result::status::SubscanModuleStatus};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/engines/yahoo.json")]
async fn run_test() {
    let mut yahoo = Yahoo::dispatcher();

    funcs::wrap_module_url(&mut yahoo, &stubr.path("/search"));

    let (results, status) = utils::run_module(yahoo, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}
