use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN};
use reqwest::Url;
use subscan::{
    enums::RequesterDispatcher,
    interfaces::module::SubscanModuleInterface,
    modules::engines::duckduckgo::{self, DUCKDUCKGO_MODULE_NAME},
    requesters::client::HTTPClient,
};
use tokio::sync::Mutex;

#[tokio::test]
#[stubr::mock("module/engines/duckduckgo.json")]
async fn duckduckgo_run_test() {
    let mut duckduckgo = duckduckgo::DuckDuckGo::new();
    let new_requester = HTTPClient::default();

    duckduckgo.requester = Mutex::new(RequesterDispatcher::HTTPClient(new_requester));
    duckduckgo.url = Url::parse(stubr.uri().as_str()).unwrap();

    let result = duckduckgo.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(duckduckgo.name().await, DUCKDUCKGO_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
