use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
};
use subscan::{
    enums::dispatchers::SubscanModuleDispatcher, interfaces::module::SubscanModuleInterface,
    modules::engines::duckduckgo::DuckDuckGo, requesters::client::HTTPClient,
};
use tokio::sync::Mutex;

#[tokio::test]
#[stubr::mock("module/engines/duckduckgo.json")]
async fn run_test() {
    let mut duckduckgo = DuckDuckGo::dispatcher();
    let new_requester = HTTPClient::default();

    funcs::wrap_module_url(&mut duckduckgo, &stubr.uri());

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut duckduckgo) = duckduckgo {
        duckduckgo.components.requester = Mutex::new(new_requester.into());
    }

    let result = duckduckgo.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}
