use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    enums::SubscanModuleDispatcher,
    interfaces::module::SubscanModuleInterface,
    modules::engines::duckduckgo::{self, DUCKDUCKGO_MODULE_NAME},
    requesters::client::HTTPClient,
};
use tokio::sync::Mutex;

#[tokio::test]
#[stubr::mock("module/engines/duckduckgo.json")]
async fn duckduckgo_run_test() {
    let mut duckduckgo = duckduckgo::DuckDuckGo::dispatcher();
    let new_requester = HTTPClient::default();

    mocks::wrap_module_dispatcher_url_field(&mut duckduckgo, &stubr.uri());

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut duckduckgo) = duckduckgo {
        duckduckgo.requester = Mutex::new(new_requester.into());
    }

    let result = duckduckgo.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(duckduckgo.name().await, DUCKDUCKGO_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}
