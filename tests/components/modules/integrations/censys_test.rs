use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};
use reqwest::Url;
use serde_json::{json, Value};
use std::env;
use subscan::{
    enums::content::Content,
    error::{
        ModuleErrorKind::JSONExtract,
        SubscanError::{self, ModuleErrorWithResult},
    },
    interfaces::module::SubscanModuleInterface,
    modules::integrations::censys::{Censys, CENSYS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/censys.json")]
async fn run_test() {
    let mut censys = Censys::dispatcher();
    let env_name = censys.envs().await.apikey.name;

    env::set_var(&env_name, "censys-api-key");
    funcs::wrap_module_url(&mut censys, &stubr.path("/censys"));

    let result = censys.run(TEST_DOMAIN).await;

    assert!(result.is_err());

    if let ModuleErrorWithResult(result) = result.err().unwrap() {
        assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
    }

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
    let json = read_stub("module/integrations/censys.json")["response"]["jsonBody"].clone();
    let extracted = Censys::extract(json, TEST_DOMAIN);
    let not_extracted = Censys::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(
        not_extracted.err().unwrap(),
        SubscanError::from(JSONExtract)
    );
}
