use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN, TEST_MODULE_NAME},
    mocks::generic_api_integration,
};
use subscan::interfaces::module::SubscanModuleInterface;

#[tokio::test]
#[stubr::mock("module/generics/api-integration.json")]
async fn generic_api_integration_run_test() {
    let mut module = generic_api_integration(&stubr.path("/subdomains"));

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(module.name().await, TEST_MODULE_NAME.to_string());
    assert_eq!(
        result,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string()
        ]
        .into()
    );
}
