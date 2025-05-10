use std::env;

use reqwest::Url;
use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::ModuleErrorKind::JSONExtract,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::certspotter::{CertSpotter, CERTSPOTTER_URL},
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/certspotter.json")]
async fn run_test() {
    let mut certspotter = CertSpotter::dispatcher();
    let env_name = certspotter.envs().await.apikey.name;

    env::set_var(&env_name, "certspotter-api-key");
    funcs::wrap_module_url(&mut certspotter, &stubr.path("/certspotter"));

    let (results, status) = utils::run_module(certspotter, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = CertSpotter::get_query_url(TEST_DOMAIN);

    let params = &[
        ("domain", TEST_DOMAIN),
        ("include_subdomains", "true"),
        ("expand", "dns_names"),
    ];

    let expected = Url::parse_with_params(CERTSPOTTER_URL, params);

    assert_eq!(url, expected.unwrap().to_string());
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = CertSpotter::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}

#[tokio::test]
async fn extract_test() {
    let stub = "module/integrations/certspotter.json";
    let json = utils::read_stub(stub)["response"]["jsonBody"].clone();

    let extracted = CertSpotter::extract(json, TEST_DOMAIN);
    let not_extracted = CertSpotter::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted.err().unwrap(), JSONExtract.into());
}
