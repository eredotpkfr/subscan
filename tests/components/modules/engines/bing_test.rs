use subscan::{modules::engines::bing::Bing, types::result::status::SubscanModuleStatus};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/engines/bing.json")]
async fn run_test() {
    let mut bing = Bing::dispatcher();

    funcs::wrap_module_url(&mut bing, &stubr.path("/search"));

    let (results, status) = utils::run_module(bing, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}
