use crate::common::{
    constants::{TEST_API_KEY, TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mocks::generic_integration,
};
use reqwest::Url;
use serde_json::Value;
use std::env;
use subscan::{
    enums::APIAuthMethod,
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
};

#[tokio::test]
async fn attribute_test() {
    let auth = APIAuthMethod::NoAuth;
    let module = generic_integration(TEST_URL, auth);
    let expected = Url::parse(TEST_URL).unwrap();

    assert_eq!(module.name().await, module.name);
    assert_eq!((module.url)(TEST_DOMAIN), expected.to_string());

    assert!(module.requester().await.is_some());
    assert!(module.extractor().await.is_some());
    assert!((module.next)(TEST_URL.parse().unwrap(), Value::Null).is_none());
}

#[tokio::test]
async fn authenticate_test_no_auth() {
    let mut url = Url::parse(TEST_URL).unwrap();
    let auth = APIAuthMethod::NoAuth;
    let module = generic_integration(&url.to_string(), auth);

    module.authenticate(&mut url, TEST_API_KEY.into()).await;

    let mut requester = module.requester().await.unwrap().lock().await;
    let rconfig = requester.config().await;

    // should not changed anything
    assert_eq!(url, url);
    assert_eq!(rconfig.headers.len(), 0)
}

#[tokio::test]
async fn authenticate_test_with_header_auth() {
    let mut url = Url::parse(TEST_URL).unwrap();
    let auth = APIAuthMethod::APIKeyAsHeader("X-API-Key".to_string());
    let module = generic_integration(&url.to_string(), auth);

    module.authenticate(&mut url, TEST_API_KEY.into()).await;

    let mut requester = module.requester().await.unwrap().lock().await;
    let rconfig = requester.config().await;

    assert_eq!(rconfig.headers.get("X-API-Key").unwrap(), TEST_API_KEY);
}

#[tokio::test]
async fn authenticate_test_with_query_auth() {
    let mut url = Url::parse(TEST_URL).unwrap();
    let expected = Url::parse_with_params(TEST_URL, &[("apikey", TEST_API_KEY)]).unwrap();

    let auth = APIAuthMethod::APIKeyAsQueryParam("apikey".to_string());
    let module = generic_integration(&url.to_string(), auth);

    module.authenticate(&mut url, TEST_API_KEY.into()).await;

    assert_eq!(url, expected);
}

#[tokio::test]
#[stubr::mock("module/generics/integration-no-auth.json")]
async fn run_test_no_auth() {
    let auth = APIAuthMethod::NoAuth;
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-header-auth.json")]
async fn run_test_with_header_auth() {
    let auth = APIAuthMethod::APIKeyAsHeader("X-API-Key".to_string());
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);

    let (env_key, _) = module.fetch_apikey().await;

    env::set_var(env_key.clone(), TEST_API_KEY);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_key);
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-query-auth.json")]
async fn run_test_with_query_auth() {
    let auth = APIAuthMethod::APIKeyAsQueryParam("apikey".to_string());
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);

    let (env_key, _) = module.fetch_apikey().await;

    env::set_var(env_key.clone(), TEST_API_KEY);

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_key);
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-query-auth.json")]
async fn run_test_with_query_auth_but_no_apikey() {
    let auth = APIAuthMethod::APIKeyAsQueryParam("apikey".to_string());
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);

    assert!(module.run(TEST_DOMAIN.to_string()).await.is_empty());
}
