use std::env;

use reqwest::Url;
use serde_json::{json, Value};
use subscan::{
    enums::content::Content,
    error::{ModuleErrorKind::JSONExtract, SubscanError::ModuleErrorWithResult},
    interfaces::module::SubscanModuleInterface,
    modules::integrations::shodan::{Shodan, SHODAN_URL},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};

#[tokio::test]
#[stubr::mock("module/integrations/shodan.json")]
async fn run_test() {
    let mut shodan = Shodan::dispatcher();
    let env_name = shodan.envs().await.apikey.name;

    env::set_var(&env_name, "shodan-api-key");
    funcs::wrap_module_url(&mut shodan, &stubr.path("/shodan"));

    let result = shodan.run(TEST_DOMAIN).await;

    assert!(result.is_err());

    if let ModuleErrorWithResult(result) = result.err().unwrap() {
        assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
    }

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Shodan::get_query_url(TEST_DOMAIN);
    let expected = format!("{SHODAN_URL}/dns/domain/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();

    let mut next = Shodan::get_next_url(url.clone(), Content::Empty);
    let mut expected = Url::parse(&format!("{TEST_URL}/?page=2")).unwrap();

    assert!(next.is_none());

    next = Shodan::get_next_url(url.clone(), json!({"more": false}).into());

    assert!(next.is_none());

    next = Shodan::get_next_url(url.clone(), json!({"more": true}).into());

    assert_eq!(next.clone().unwrap(), expected);

    next = Shodan::get_next_url(next.unwrap(), json!({"more": true}).into());
    expected = Url::parse(&format!("{TEST_URL}/?page=3")).unwrap();

    assert_eq!(next.unwrap(), expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/shodan.json")["response"]["jsonBody"].clone();
    let extracted = Shodan::extract(json, TEST_DOMAIN);
    let not_extracted = Shodan::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
