use std::env;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs, mocks,
};
use reqwest::Url;
use serde_json::{json, Value};
use subscan::{
    enums::{content::Content, dispatchers::SubscanModuleDispatcher},
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
async fn run_test() {
    let (stubs, templates) = (
        "module/integrations/github",
        vec!["github-code-search-template.json"],
    );

    let (tmp, port) = funcs::create_tmp_stubs_with_port(stubs, templates);

    let config = stubr::Config {
        port: Some(port),
        ..Default::default()
    };

    let stubr = stubr::Stubr::start_with(tmp.clone(), config).await;
    let mut github = GitHub::dispatcher();
    let env_name = github.envs().await.apikey.name;

    env::set_var(&env_name, "github-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut github, &stubr.path("/github-code-search"));

    let results = github.run(TEST_DOMAIN).await;

    assert_eq!(results.subdomains, [TEST_BAR_SUBDOMAIN.to_string()].into());

    tokio::fs::remove_dir_all(tmp).await.unwrap();
    env::remove_var(env_name);
}
