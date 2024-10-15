use std::collections::BTreeSet;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::waybackarchive::WaybackArchive,
};

#[tokio::test]
#[stubr::mock("module/integrations/waybackarchive.json")]
async fn run_test() {
    let mut waybackarchive = WaybackArchive::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut waybackarchive, &stubr.path("/waybackarchive"));

    let results = waybackarchive.run(TEST_DOMAIN).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert_eq!(results, expected);
}
