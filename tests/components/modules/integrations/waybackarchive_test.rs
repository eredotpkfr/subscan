use std::{collections::BTreeSet, time::Duration};

use subscan::{
    error::ModuleErrorKind::GetContent, modules::integrations::waybackarchive::WaybackArchive,
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
#[stubr::mock("module/integrations/waybackarchive/waybackarchive.json")]
async fn run_success_test() {
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

#[tokio::test]
#[stubr::mock("module/integrations/waybackarchive/waybackarchive-delayed.json")]
async fn run_timeout_test() {
    let mut waybackarchive = WaybackArchive::dispatcher();

    funcs::wrap_module_url(&mut waybackarchive, &stubr.path("/waybackarchive-delayed"));
    funcs::set_requester_timeout(&mut waybackarchive, Duration::from_millis(500)).await;

    let (results, status) = utils::run_module(waybackarchive, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, GetContent.into());
}
