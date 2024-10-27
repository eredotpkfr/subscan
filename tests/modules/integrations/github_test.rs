use std::env;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use reqwest::Url;
use serde_json::{json, Value};
use subscan::{
    enums::{Content, SubscanModuleDispatcher},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::github::GitHub,
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

        assert_eq!(module.get_html_urls(Content::Empty).await, None);
        assert_eq!(
            module.get_html_urls(json.into()).await.unwrap(),
            [expected].into()
        );
    }
}

#[tokio::test]
#[stubr::mock("module/integrations/github", port = 2222)]
async fn run_test() {
    let mut github = GitHub::dispatcher();
    let env_name = github.envs().await.apikey.name;

    env::set_var(&env_name, "github-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut github, &stubr.path("/github-code-search"));

    let results = github.run(TEST_DOMAIN).await;

    assert_eq!(results.subdomains, [TEST_BAR_SUBDOMAIN.to_string()].into());

    env::remove_var(env_name);
}
