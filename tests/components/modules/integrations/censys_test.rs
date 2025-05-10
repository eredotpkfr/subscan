use std::env;

use reqwest::Url;
use serde_json::{json, Value};
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtract,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::censys::{Censys, CENSYS_URL},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/censys.json")]
async fn run_test() {
    let mut censys = Censys::dispatcher();
    let env_name = censys.envs().await.apikey.name;

    env::set_var(&env_name, "censys-api-key");
    funcs::wrap_module_url(&mut censys, &stubr.path("/censys"));

    let (results, status) = utils::run_module(censys, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, JSONExtract.into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Censys::get_query_url(TEST_DOMAIN);
    let expected = format!("{CENSYS_URL}?q={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();
    let json = json!({"result": {"links": {"next": "foo"}}});
    let expected = Url::parse(&format!("{TEST_URL}/?cursor=foo")).unwrap();

    let mut next = Censys::get_next_url(url.clone(), Content::Empty);

    assert!(next.is_none());

    next = Censys::get_next_url(url, json.into());

    assert_eq!(next.unwrap(), expected);
}

#[tokio::test]
async fn extract_test() {
    let stub = "module/integrations/censys.json";
    let json = utils::read_stub(stub)["response"]["jsonBody"].clone();

    let extracted = Censys::extract(json, TEST_DOMAIN);
    let not_extracted = Censys::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
