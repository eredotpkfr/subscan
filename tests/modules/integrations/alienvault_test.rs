use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use subscan::{interfaces::module::SubscanModuleInterface, modules::integrations::alienvault};

#[tokio::test]
#[stubr::mock("module/integrations/alienvault.json")]
async fn alienvault_run_test() {
    let mut alienvault = alienvault::AlienVault::new();
    let url = Url::parse(stubr.path("/alienvault").as_str()).unwrap();

    alienvault.url = Box::new(move |_| url.to_string());

    let result = alienvault.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(alienvault.name().await, "AlienVault");
    assert_eq!(
        result,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string(),
        ]
        .into()
    );
}
