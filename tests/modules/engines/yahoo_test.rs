use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::engines::yahoo::{self, YAHOO_MODULE_NAME},
};

#[tokio::test]
#[stubr::mock("module/engines/yahoo.json")]
async fn yahoo_run_test() {
    let mut yahoo = yahoo::Yahoo::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut yahoo, &stubr.path("/search"));

    let result = yahoo.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(yahoo.name().await, YAHOO_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
