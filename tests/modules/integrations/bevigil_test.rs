use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use serde_json::{self, Value};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::bevigil::{self, BEVIGIL_MODULE_NAME, BEVIGIL_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/bevigil.json")]
async fn bevigil_run_test() {
    let mut bevigil = bevigil::Bevigil::new();
    let url = Url::parse(stubr.path("/bevigil").as_str()).unwrap();

    bevigil.url = Box::new(move |_| url.to_string());

    let result = bevigil.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(bevigil.name().await, BEVIGIL_MODULE_NAME);
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
    let url = bevigil::Bevigil::get_query_url(TEST_DOMAIN.to_string());
    let expected = format!("{BEVIGIL_URL}/{TEST_DOMAIN}/subdomains");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = "{\"subdomains\": [\"bar.foo.com\"]}";

    let extracted = bevigil::Bevigil::extract(serde_json::from_str(json).unwrap());
    let not_extracted = bevigil::Bevigil::extract(Value::default());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, [].into());
}
