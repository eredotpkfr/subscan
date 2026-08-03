use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use subscan::utilities::net::read_resolver_list_file;

use crate::common::utils::{tcp_ns, testdata_path};

#[tokio::test]
pub async fn test_read_resolver_list_file() {
    let path = testdata_path().join("txt/resolverlist.txt");
    let config = read_resolver_list_file(path);

    let expected = [
        tcp_ns(IpAddr::V4(Ipv4Addr::from_str("127.0.0.1").unwrap()), 1),
        tcp_ns(IpAddr::V4(Ipv4Addr::from_str("127.9.2.129").unwrap()), 25),
        tcp_ns(
            IpAddr::V4(Ipv4Addr::from_str("176.255.45.12").unwrap()),
            123,
        ),
        tcp_ns(IpAddr::V4(Ipv4Addr::from_str("192.168.1.1").unwrap()), 8080),
        tcp_ns(
            IpAddr::V4(Ipv4Addr::from_str("10.126.125.98").unwrap()),
            4444,
        ),
        tcp_ns(IpAddr::V4(Ipv4Addr::from_str("0.0.0.0").unwrap()), 4444),
        tcp_ns(IpAddr::V6(Ipv6Addr::from_str("2001:db8::1").unwrap()), 8080),
        tcp_ns(
            IpAddr::V6(Ipv6Addr::from_str("2001:db8:85a3:8d3:1319:8a2e:370:7348").unwrap()),
            443,
        ),
        tcp_ns(
            IpAddr::V6(Ipv6Addr::from_str("abcd:ef::42:1").unwrap()),
            8080,
        ),
        tcp_ns(
            IpAddr::V6(Ipv6Addr::from_str("::ffff:1.2.3.4").unwrap()),
            4444,
        ),
        tcp_ns(IpAddr::V6(Ipv6Addr::from_str("::1").unwrap()), 1234),
        tcp_ns(IpAddr::V6(Ipv6Addr::from_str("::c0a8:1e02").unwrap()), 8001),
    ];

    assert!(config.domain().is_none());
    assert!(config.search().is_empty());

    for item in config.name_servers().iter().zip(&expected) {
        assert_eq!(item.0.ip, item.1.ip);
        assert_eq!(
            item.0.trust_negative_responses,
            item.1.trust_negative_responses
        );
        assert_eq!(item.0.connections.len(), item.1.connections.len());

        for conn in item.0.connections.iter().zip(&item.1.connections) {
            assert_eq!(conn.0.port, conn.1.port);
            assert_eq!(conn.0.protocol, conn.1.protocol);
        }
    }
}
