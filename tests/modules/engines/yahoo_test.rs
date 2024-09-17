use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::yahoo};

#[tokio::test]
#[stubr::mock("module/engines/yahoo.json")]
async fn yahoo_run_test() {
    let mut yahoo = yahoo::Yahoo::new();

    yahoo.url = Url::parse(stubr.path("/search").as_str()).unwrap();

    let result = yahoo.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(yahoo.name().await, "Yahoo");
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
