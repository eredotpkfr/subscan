use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use subscan::{
    enums::RequesterDispatcher, interfaces::module::SubscanModuleInterface,
    modules::engines::duckduckgo, requesters::client::HTTPClient,
};

#[tokio::test]
#[stubr::mock("module/engines/duckduckgo.json")]
async fn duckduckgo_run_test() {
    let mut duckduckgo = duckduckgo::DuckDuckGo::new();
    let new_requester = HTTPClient::default();

    duckduckgo.requester = RequesterDispatcher::HTTPClient(new_requester).into();
    duckduckgo.url = Url::parse(stubr.uri().as_str()).unwrap();

    let result = duckduckgo.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(duckduckgo.name().await, "DuckDuckGo");
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
