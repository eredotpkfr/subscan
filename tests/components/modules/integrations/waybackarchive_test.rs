use std::collections::BTreeSet;

use subscan::{
    modules::integrations::waybackarchive::WaybackArchive,
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/waybackarchive.json")]
async fn run_test() {
    let mut waybackarchive = WaybackArchive::dispatcher();

    funcs::wrap_module_url(&mut waybackarchive, &stubr.path("/waybackarchive"));

    let (results, status) = utils::run_module(waybackarchive, TEST_DOMAIN).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert_eq!(results, expected);
    assert_eq!(status, SubscanModuleStatus::Finished);
}
