use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    funcs::read_stub,
    mocks,
};
use reqwest::Url;
use serde_json::{json, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    enums::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::virustotal::{VirusTotal, VIRUSTOTAL_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/virustotal.json")]
async fn run_test() {
    let mut virustotal = VirusTotal::dispatcher();
    let env_name = virustotal.envs().await.apikey.name;

    env::set_var(&env_name, "virustotal-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut virustotal, &stubr.path("/virustotal"));

    let result = virustotal.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = VirusTotal::get_query_url(TEST_DOMAIN);
    let expected = format!("{VIRUSTOTAL_URL}/{TEST_DOMAIN}/subdomains?limit=250");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();
    let expected = Url::parse("https://bar.com").unwrap();
    let json = json!({"links": {"next": "https://bar.com"}});

    let mut next = VirusTotal::get_next_url(url.clone(), Content::Empty);

    assert!(next.is_none());

    next = VirusTotal::get_next_url(url, json.into());

    assert_eq!(next.unwrap(), expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/virustotal.json")["response"]["jsonBody"].clone();
    let extracted = VirusTotal::extract(json, TEST_DOMAIN);
    let not_extracted = VirusTotal::extract(Value::Null, TEST_DOMAIN);

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
