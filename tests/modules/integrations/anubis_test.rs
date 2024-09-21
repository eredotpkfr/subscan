use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use subscan::{interfaces::module::SubscanModuleInterface, modules::integrations::anubis};

#[tokio::test]
#[stubr::mock("module/integrations/anubis.json")]
async fn anubis_run_test() {
    let mut anubis = anubis::Anubis::new();
    let url = Url::parse(stubr.path("/anubis").as_str()).unwrap();

    anubis.url = Box::new(move |_| url.to_string());

    let result = anubis.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(anubis.name().await, "Anubis");
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
