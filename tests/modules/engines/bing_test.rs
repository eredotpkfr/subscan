use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::engines::bing::{self, BING_MODULE_NAME},
};

#[tokio::test]
#[stubr::mock("module/engines/bing.json")]
async fn bing_run_test() {
    let mut bing = bing::Bing::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut bing, &stubr.path("/search"));

    let result = bing.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(bing.name().await, BING_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}
