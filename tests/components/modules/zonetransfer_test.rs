use std::{
    collections::BTreeSet,
    net::{SocketAddr, SocketAddrV4},
    str::FromStr,
};

use hickory_client::proto::xfer::Protocol;
use hickory_resolver::config::NameServerConfig;
use subscan::{
    enums::dispatchers::SubscanModuleDispatcher, modules::zonetransfer::ZoneTransfer,
    types::result::status::SubscanModuleStatus,
};

use crate::common::{
    constants::{LOCAL_HOST, TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
    utils,
};

#[tokio::test]
async fn get_tcp_client_test() {
    let server = funcs::spawn_mock_dns_server().await;
    let zonetransfer = ZoneTransfer::dispatcher();

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        assert!(zonetransfer.get_tcp_client(server.socket).await.is_ok());
    }
}

#[tokio::test]
async fn get_tcp_client_fail_test() {
    let zonetransfer = ZoneTransfer::dispatcher();
    let addr = SocketAddr::from_str(&format!("{LOCAL_HOST}:0")).unwrap();

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        assert!(zonetransfer.get_tcp_client(addr).await.is_err());
    }
}

#[tokio::test]
async fn get_ns_as_ip_test() {
    let server = funcs::spawn_mock_dns_server().await;
    let zonetransfer = ZoneTransfer::dispatcher();

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        let ips = zonetransfer.get_ns_as_ip(server.socket, TEST_DOMAIN).await;

        assert_eq!(ips.unwrap(), [server.socket]);
    }
}

#[tokio::test]
async fn attempt_zone_transfer_test() {
    let server = funcs::spawn_mock_dns_server().await;
    let zonetransfer = ZoneTransfer::dispatcher();

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        let subs = zonetransfer.attempt_zone_transfer(server.socket, TEST_DOMAIN).await;

        assert_eq!(subs.unwrap(), [TEST_BAR_SUBDOMAIN]);
    }
}

#[tokio::test]
async fn run_success_test() {
    let server = funcs::spawn_mock_dns_server().await;
    let mut zonetransfer = ZoneTransfer::dispatcher();

    if let SubscanModuleDispatcher::ZoneTransfer(ref mut zonetransfer) = zonetransfer {
        zonetransfer.ns = Some(NameServerConfig::new(server.socket, Protocol::Tcp));
    }

    let (results, status) = utils::run_module(zonetransfer, TEST_DOMAIN).await;

    assert_eq!(results, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(status, SubscanModuleStatus::Finished);
}

#[tokio::test]
async fn run_no_default_ns_test() {
    let mut zonetransfer = ZoneTransfer::dispatcher();

    if let SubscanModuleDispatcher::ZoneTransfer(ref mut zonetransfer) = zonetransfer {
        zonetransfer.ns = None;
    }

    let (results, status) = utils::run_module(zonetransfer, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, "no default ns".into())
}

#[tokio::test]
async fn run_failed_test() {
    let mut zonetransfer = ZoneTransfer::dispatcher();
    let socketaddr = SocketAddr::V4(SocketAddrV4::from_str("0.0.0.0:0").unwrap());

    if let SubscanModuleDispatcher::ZoneTransfer(ref mut zonetransfer) = zonetransfer {
        zonetransfer.ns = Some(NameServerConfig::new(socketaddr, Protocol::Tcp));
    }

    let (results, status) = utils::run_module(zonetransfer, TEST_DOMAIN).await;

    assert_eq!(results, BTreeSet::new());
    assert_eq!(status, "connection error".into())
}
