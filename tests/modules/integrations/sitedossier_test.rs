use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mocks,
};
use serde_json::Value;
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::sitedossier::{Sitedossier, SITEDOSSIER_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/sitedossier.json")]
async fn run_test() {
    let mut sitedossier = Sitedossier::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut sitedossier, &stubr.path("/sitedossier"));

    let result = sitedossier.run(TEST_DOMAIN).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Sitedossier::get_query_url(TEST_DOMAIN);
    let expected = format!("{SITEDOSSIER_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = Sitedossier::get_next_url(url, Value::Null);

    assert!(next.is_none());
}
