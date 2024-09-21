use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use serde_json::{self, Value};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::alienvault::{self, ALIENVAULT_MODULE_NAME, ALIENVAULT_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/alienvault.json")]
async fn alienvault_run_test() {
    let mut alienvault = alienvault::AlienVault::new();
    let url = Url::parse(stubr.path("/alienvault").as_str()).unwrap();

    alienvault.url = Box::new(move |_| url.to_string());

    let result = alienvault.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(alienvault.name().await, ALIENVAULT_MODULE_NAME);
    assert_eq!(
        result,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string(),
        ]
        .into()
    );
}

#[tokio::test]
async fn get_query_url_test() {
    let url = alienvault::AlienVault::get_query_url(TEST_DOMAIN.to_string());
    let expected = format!("{ALIENVAULT_URL}/{TEST_DOMAIN}/passive_dns");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = "{\"passive_dns\": [{\"hostname\": \"bar.foo.com\"}]}";

    let extracted = alienvault::AlienVault::extract(serde_json::from_str(json).unwrap());
    let not_extracted = alienvault::AlienVault::extract(Value::default());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, [].into());
}
