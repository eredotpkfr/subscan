use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::engines::google::{self, GOOGLE_MODULE_NAME},
};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn google_run_test() {
    let mut google = google::Google::new();

    google.url = Url::parse(stubr.path("/search").as_str()).unwrap();

    let result = google.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(google.name().await, GOOGLE_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
