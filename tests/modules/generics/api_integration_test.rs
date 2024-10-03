use crate::common::{
    constants::{TEST_API_KEY, TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks::generic_api_integration,
};
use std::env;
use subscan::{enums::APIAuthMethod, interfaces::module::SubscanModuleInterface};

#[tokio::test]
#[stubr::mock("module/generics/api-integration-no-auth.json")]
async fn generic_api_integration_run_test_no_auth() {
    let auth = APIAuthMethod::NoAuth;
    let mut module = generic_api_integration(&stubr.path("/subdomains"), auth);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(module.name().await, module.name);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
#[stubr::mock("module/generics/api-integration-with-header-auth.json")]
async fn generic_api_integration_run_test_with_header_auth() {
    let auth = APIAuthMethod::APIKeyAsHeader("X-API-Key".to_string());
    let mut module = generic_api_integration(&stubr.path("/subdomains"), auth);

    let (env_key, _) = module.fetch_apikey().await;

    env::set_var(env_key.clone(), TEST_API_KEY);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(module.name().await, module.name);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());

    env::remove_var(env_key);
}

#[tokio::test]
#[stubr::mock("module/generics/api-integration-with-query-auth.json")]
async fn generic_api_integration_run_test_with_url_auth() {
    let auth = APIAuthMethod::APIKeyAsQueryParam("apikey".to_string());
    let mut module = generic_api_integration(&stubr.path("/subdomains"), auth);

    let (env_key, _) = module.fetch_apikey().await;

    env::set_var(env_key.clone(), TEST_API_KEY);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(module.name().await, module.name);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());

    env::remove_var(env_key);
}
