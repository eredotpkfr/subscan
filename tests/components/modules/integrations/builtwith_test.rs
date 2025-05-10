use std::env;

use reqwest::Url;
use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtract,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::builtwith::{BuiltWith, BUILTWITH_URL},
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/builtwith.json")]
async fn run_test() {
    let mut builtwith = BuiltWith::dispatcher();
    let env_name = builtwith.envs().await.apikey.name;

    env::set_var(&env_name, "builtwith-api-key");
    funcs::wrap_module_url(&mut builtwith, &stubr.path("/builtwith"));

    let (results, status) = utils::run_module(builtwith, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let params = &[
        ("HIDETEXT", "yes"),
        ("HIDEDL", "yes"),
        ("NOLIVE", "yes"),
        ("NOMETA", "yes"),
        ("NOPII", "yes"),
        ("NOATTR", "yes"),
        ("LOOKUP", TEST_DOMAIN),
    ];

    let expected = Url::parse_with_params(BUILTWITH_URL, params).unwrap();
    let url = BuiltWith::get_query_url(TEST_DOMAIN);

    assert_eq!(url, expected.to_string());
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = BuiltWith::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let stub = "module/integrations/builtwith.json";
    let json = utils::read_stub(stub)["response"]["jsonBody"].clone();

    let extracted = BuiltWith::extract(json, TEST_DOMAIN);
    let not_extracted = BuiltWith::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
