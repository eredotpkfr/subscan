use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::yahoo::Yahoo};

#[tokio::test]
#[stubr::mock("module/engines/yahoo.json")]
async fn run_test() {
    let mut yahoo = Yahoo::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut yahoo, &stubr.path("/search"));

    let result = yahoo.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}
