use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use serde_json::{self, Value};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::anubis::{self, ANUBIS_MODULE_NAME, ANUBIS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/anubis.json")]
async fn anubis_run_test() {
    let mut anubis = anubis::Anubis::new();
    let url = Url::parse(stubr.path("/anubis").as_str()).unwrap();

    anubis.url = Box::new(move |_| url.to_string());

    let result = anubis.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(anubis.name().await, ANUBIS_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = anubis::Anubis::get_query_url(TEST_DOMAIN.to_string());
    let expected = format!("{ANUBIS_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = "[\"bar.foo.com\"]";

    let extracted = anubis::Anubis::extract(serde_json::from_str(json).unwrap());
    let not_extracted = anubis::Anubis::extract(Value::default());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, [].into());
}
