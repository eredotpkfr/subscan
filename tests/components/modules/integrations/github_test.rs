use std::{collections::BTreeSet, env, time::Duration};

use reqwest::Url;
use serde_json::{json, Value};
use subscan::{
    enums::{content::Content, dispatchers::SubscanModuleDispatcher},
    error::ModuleErrorKind::GetContent,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::github::GitHub,
    types::result::status::{SkipReason::AuthenticationNotProvided, SubscanModuleStatus},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    stub::StubTemplateManager,
    utils,
};

#[tokio::test]
async fn get_raw_url_test() {
    if let SubscanModuleDispatcher::GitHub(module) = GitHub::dispatcher() {
        let json = json!({"html_url": "https://github.com/foo/blob/bar"});
        let expected = Url::parse("https://raw.githubusercontent.com/foo/bar").unwrap();

        assert_eq!(module.get_raw_url(&Value::Null), None);
        assert_eq!(module.get_raw_url(&json).unwrap(), expected);
    }
}

#[tokio::test]
async fn get_html_urls_test() {
    if let SubscanModuleDispatcher::GitHub(module) = GitHub::dispatcher() {
        let json = json!({"items": [{"html_url": "https://github.com/foo/blob/bar"}]});
        let expected = Url::parse("https://raw.githubusercontent.com/foo/bar").unwrap();
        let results = module.get_html_urls(json.into()).await.unwrap();

        assert_eq!(module.get_html_urls(Content::Empty).await, None);
        assert_eq!(results, [expected].into());
    }
}

#[tokio::test]
async fn run_success_test() {
    let stubs = "module/integrations/github";
    let templates = vec!["github-code-search-template.json"];
    let manager: StubTemplateManager = (stubs, templates).into();

    let config = stubr::Config {
        port: Some(manager.port().await),
        ..Default::default()
    };

    let stubr = stubr::Stubr::start_with(manager.temp().await, config).await;
    let mut github = GitHub::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut github, utils::current_thread_hex());

    let env_name = github.envs().await.apikey.name;

    env::set_var(&env_name, "github-api-key");
    funcs::wrap_module_url(&mut github, &stubr.path("/github-code-search"));

    let (results, status) = utils::run_module(github, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/integrations/github")]
async fn run_not_authenticated_test() {
    let mut github = GitHub::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut github, utils::current_thread_hex());
    funcs::wrap_module_url(&mut github, &stubr.path("/github-code-search"));

    let (results, status) = utils::run_module(github, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, AuthenticationNotProvided.into());
}

#[tokio::test]
#[stubr::mock("module/integrations/github/github-code-search-no-data.json")]
async fn run_failed_test() {
    let mut github = GitHub::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut github, utils::current_thread_hex());

    let env_name = github.envs().await.apikey.name;

    env::set_var(&env_name, "github-api-key");
    funcs::wrap_module_url(&mut github, &stubr.path("/github-code-search-no-data"));

    let (results, status) = utils::run_module(github, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, "not get raw URLs".into());

    env::remove_var(env_name);
}

#[tokio::test]
#[stubr::mock("module/integrations/github/github-code-search-delayed.json")]
async fn run_timeout_test() {
    let mut github = GitHub::dispatcher();

    // Izolate non-generic module environment variables
    funcs::wrap_module_name(&mut github, utils::current_thread_hex());

    let env_name = github.envs().await.apikey.name;

    env::set_var(&env_name, "github-api-key");
    funcs::wrap_module_url(&mut github, &stubr.path("/github-code-search-delayed"));
    funcs::set_requester_timeout(&mut github, Duration::from_millis(500)).await;

    let (results, status) = utils::run_module(github, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, GetContent.into());

    env::remove_var(env_name);
}
