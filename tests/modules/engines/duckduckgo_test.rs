use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use subscan::{
    cache::requesters, enums::RequesterType, interfaces::module::SubscanModuleInterface,
    modules::engines::duckduckgo,
};

#[tokio::test]
#[stubr::mock("module/engines/duckduckgo.json")]
async fn duckduckgo_run_test() {
    let mut duckduckgo = duckduckgo::DuckDuckGo::new();

    duckduckgo.requester = requesters::get_by_type(&RequesterType::HTTPClient);
    duckduckgo.url = Url::parse(stubr.uri().as_str()).unwrap();

    let result = duckduckgo.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(duckduckgo.name().await, "DuckDuckGo");
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
