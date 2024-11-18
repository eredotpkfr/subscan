use crate::common::{
    constants::{TEST_API_KEY, TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::modules::generic_integration,
};
use reqwest::Url;
use std::env;
use subscan::{
    enums::{auth::AuthenticationMethod, content::Content},
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    types::{
        env::{Credentials, Env},
        result::status::SkipReason::AuthenticationNotProvided,
    },
};

#[tokio::test]
async fn attribute_test() {
    let auth = AuthenticationMethod::NoAuthentication;
    let module = generic_integration(TEST_URL, auth);
    let envs = module.envs().await;
    let expected = Url::parse(TEST_URL).unwrap();

    assert_eq!((module.funcs.url)(TEST_DOMAIN), expected.to_string());
    assert_eq!(module.name().await, module.name);

    assert!(envs.apikey.value.is_none());
    assert!(envs.host.value.is_none());
    assert!(envs.credentials.username.value.is_none());
    assert!(envs.credentials.password.value.is_none());

    assert!(module.requester().await.is_some());
    assert!(module.extractor().await.is_some());

    assert!((module.funcs.next)(TEST_URL.parse().unwrap(), Content::Empty).is_none());
}

#[tokio::test]
async fn authenticate_test_no_auth() {
    let mut url = Url::parse(TEST_URL).unwrap();

    let auth = AuthenticationMethod::NoAuthentication;
    let module = generic_integration(url.as_ref(), auth);

    assert!(module.authenticate(&mut url).await);

    let requester = &mut *module.requester().await.unwrap().lock().await;
    let rconfig = requester.config().await;

    // Should be not changed anything
    assert_eq!(url, url);
    assert_eq!(rconfig.headers.len(), 0)
}

#[tokio::test]
async fn authenticate_test_with_header_auth() {
    let mut url = Url::parse(TEST_URL).unwrap();

    let auth = AuthenticationMethod::APIKeyAsHeader("X-API-Key".to_string());
    let module = generic_integration(url.as_ref(), auth);

    let envs = module.envs().await;
    let env_var = envs.apikey.name;

    // Set the API key
    env::set_var(env_var.clone(), TEST_API_KEY);

    // Should be authenticated
    assert!(module.authenticate(&mut url).await);

    let requester = &mut *module.requester().await.unwrap().lock().await;
    let rconfig = requester.config().await;

    // Check if the API key set on headers
    assert_eq!(rconfig.headers.get("X-API-Key").unwrap(), TEST_API_KEY);

    env::remove_var(env_var);
}

#[tokio::test]
async fn authenticate_test_with_header_auth_no_apikey() {
    let mut url = Url::parse(TEST_URL).unwrap();

    let auth = AuthenticationMethod::APIKeyAsHeader("X-API-Key".to_string());
    let module = generic_integration(url.as_ref(), auth);

    // There is no API key in environments, should be not authenticated
    assert!(!module.authenticate(&mut url).await);
}

#[tokio::test]
async fn authenticate_test_with_query_auth() {
    let mut url = Url::parse(TEST_URL).unwrap();

    let expected = Url::parse_with_params(TEST_URL, &[("apikey", TEST_API_KEY)]).unwrap();
    let auth = AuthenticationMethod::APIKeyAsQueryParam("apikey".to_string());
    let module = generic_integration(url.as_ref(), auth);

    let envs = module.envs().await;
    let env_var = envs.apikey.name;

    // Set the API key
    env::set_var(env_var.clone(), TEST_API_KEY);

    // Should be authenticated
    assert!(module.authenticate(&mut url).await);

    // Check if the query parameter adds on URL
    assert_eq!(url, expected);

    env::remove_var(env_var);
}

#[tokio::test]
async fn authenticate_test_with_query_auth_no_apikey() {
    let mut url = Url::parse(TEST_URL).unwrap();

    let auth = AuthenticationMethod::APIKeyAsQueryParam("apikey".to_string());
    let module = generic_integration(url.as_ref(), auth);

    // There is no API key in environments, should be not authenticated
    assert!(!module.authenticate(&mut url).await);
}

#[tokio::test]
async fn authenticate_test_with_basic_http_auth_from_credentials() {
    let mut url = Url::parse(TEST_URL).unwrap();

    // Set credentials on startup
    let credentials = Credentials {
        username: Env {
            name: "USERNAME".into(),
            value: Some("foo".to_string()),
        },
        password: Env {
            name: "PASSWORD".into(),
            value: Some("bar".to_string()),
        },
    };
    let auth = AuthenticationMethod::BasicHTTPAuthentication(credentials.clone());
    let module = generic_integration(url.as_ref(), auth);

    // Should be authenticated
    assert!(module.authenticate(&mut url).await);

    let requester = &mut *module.requester().await.unwrap().lock().await;

    // Check if the credentials set on requester instance
    assert_eq!(requester.config().await.credentials, credentials);
}

#[tokio::test]
async fn authenticate_test_with_basic_http_auth_from_envs() {
    let mut url = Url::parse(TEST_URL).unwrap();

    // There is no credentials on startup, it should be authenticated with using
    // environment variables
    let credentials = Credentials::default();
    let auth = AuthenticationMethod::BasicHTTPAuthentication(credentials);
    let module = generic_integration(url.as_ref(), auth);
    let envs = module.envs().await;

    // Set credentials as a environment
    env::set_var(envs.credentials.username.name.clone(), "foo");
    env::set_var(envs.credentials.password.name.clone(), "bar");

    // Should be authenticated
    assert!(module.authenticate(&mut url).await);

    let requester = &mut *module.requester().await.unwrap().lock().await;
    let rconfig = requester.config().await;

    // Check if the credentials set on requester instance
    assert_eq!(rconfig.credentials.username.value.clone().unwrap(), "foo");
    assert_eq!(rconfig.credentials.password.value.clone().unwrap(), "bar");

    env::remove_var(envs.credentials.username.name);
    env::remove_var(envs.credentials.password.name);
}

#[tokio::test]
async fn authenticate_test_with_basic_http_auth_no_credentials() {
    let mut url = Url::parse(TEST_URL).unwrap();

    let credentials = Credentials::default();
    let auth = AuthenticationMethod::BasicHTTPAuthentication(credentials);
    let module = generic_integration(url.as_ref(), auth);

    // There is no credentials both startup and environments, should be not authenticated
    assert!(!module.authenticate(&mut url).await);
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-no-auth.json")]
async fn run_test_no_auth() {
    let auth = AuthenticationMethod::NoAuthentication;
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);

    let result = module.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-header-auth.json")]
async fn run_test_with_header_auth() {
    let auth = AuthenticationMethod::APIKeyAsHeader("X-API-Key".to_string());
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);

    let env_name = module.envs().await.apikey.name;

    env::set_var(env_name.clone(), TEST_API_KEY);

    let result = module.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-query-auth.json")]
async fn run_test_with_query_auth() {
    let auth = AuthenticationMethod::APIKeyAsQueryParam("apikey".to_string());
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);

    let env_name = module.envs().await.apikey.name;

    env::set_var(env_name.clone(), TEST_API_KEY);

    let result = module.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-basic-http-auth.json")]
async fn run_test_with_basic_http_auth() {
    let credentials = Credentials {
        username: Env {
            name: "USERNAME".into(),
            value: Some("foo".to_string()),
        },
        password: Env {
            name: "PASSWORD".into(),
            value: Some("bar".to_string()),
        },
    };

    let auth = AuthenticationMethod::BasicHTTPAuthentication(credentials);
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);
    let result = module.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-query-auth.json")]
async fn run_test_with_not_authenticated() {
    let auth = AuthenticationMethod::APIKeyAsQueryParam("apikey".to_string());
    let mut module = generic_integration(&stubr.path("/subdomains"), auth);
    let result = module.run(TEST_DOMAIN).await;

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), AuthenticationNotProvided.into());
}
