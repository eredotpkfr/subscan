use crate::common::{
    constants::{
        TEST_API_KEY, TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN, TEST_MODULE_NAME,
    },
    mocks::generic_api_integration,
};
use std::env;
use subscan::{enums::AuthMethod, interfaces::module::SubscanModuleInterface};

#[tokio::test]
#[stubr::mock("module/generics/api-integration-no-auth.json")]
async fn generic_api_integration_run_test_no_auth() {
    let auth = AuthMethod::NoAuth;
    let mut module = generic_api_integration(&stubr.path("/subdomains"), auth);

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

#[tokio::test]
#[stubr::mock("module/generics/api-integration-with-header-auth.json")]
async fn generic_api_integration_run_test_with_header_auth() {
    env::set_var("SUBSCAN_FOO_APIKEY", TEST_API_KEY);

    let auth = AuthMethod::APIKeyInHeader("X-API-Key".to_string());
    let mut module = generic_api_integration(&stubr.path("/subdomains"), auth);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(module.name().await, TEST_MODULE_NAME.to_string());
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}

#[tokio::test]
#[stubr::mock("module/generics/api-integration-with-url-auth.json")]
async fn generic_api_integration_run_test_with_url_auth() {
    let auth = AuthMethod::APIKeyInURL;
    let url = format!("{}?apikey={}", stubr.path("/subdomains"), TEST_API_KEY);

    let mut module = generic_api_integration(&url, auth);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(module.name().await, TEST_MODULE_NAME.to_string());
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
