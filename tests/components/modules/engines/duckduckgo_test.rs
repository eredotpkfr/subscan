use subscan::{
    enums::dispatchers::SubscanModuleDispatcher, modules::engines::duckduckgo::DuckDuckGo,
    requesters::client::HTTPClient, types::result::status::SubscanModuleStatus,
};
use tokio::sync::Mutex;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/engines/duckduckgo.json")]
async fn run_test() {
    let mut duckduckgo = DuckDuckGo::dispatcher();
    let new_requester = HTTPClient::default();

    funcs::wrap_module_url(&mut duckduckgo, &stubr.uri());

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut duckduckgo) = duckduckgo {
        duckduckgo.components.requester = Mutex::new(new_requester.into());
    }

    let (results, status) = utils::run_module(duckduckgo, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}
