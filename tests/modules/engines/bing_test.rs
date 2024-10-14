use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::bing::Bing};

#[tokio::test]
#[stubr::mock("module/engines/bing.json")]
async fn run_test() {
    let mut bing = Bing::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut bing, &stubr.path("/search"));

    let result = bing.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}
