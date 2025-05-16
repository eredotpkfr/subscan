use std::env;

use reqwest::Url;
use serde_json::{json, Value};
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtract,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::virustotal::{VirusTotal, VIRUSTOTAL_URL},
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/virustotal.json")]
async fn run_test() {
    let mut virustotal = VirusTotal::dispatcher();
    let env_name = virustotal.envs().await.apikey.name;

    env::set_var(&env_name, "virustotal-api-key");
    funcs::wrap_module_url(&mut virustotal, &stubr.path("/virustotal"));

    let (results, status) = utils::run_module(virustotal, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);

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
    let stub = "module/integrations/virustotal.json";
    let json = utils::read_stub(stub)["response"]["jsonBody"].clone();

    let extracted = VirusTotal::extract(json, TEST_DOMAIN);
    let not_extracted = VirusTotal::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
