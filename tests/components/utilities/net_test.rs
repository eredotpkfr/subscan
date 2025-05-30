use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use hickory_client::proto::xfer::Protocol::Tcp;
use hickory_resolver::config::NameServerConfig;
use subscan::utilities::net::read_resolver_list_file;

use crate::common::utils::testdata_path;

#[tokio::test]
pub async fn test_read_resolver_list_file() {
    let path = testdata_path().join("txt/resolverlist.txt");
    let config = read_resolver_list_file(path);

    let expected = [
        NameServerConfig::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str("127.0.0.1").unwrap()), 1),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str("127.9.2.129").unwrap()), 25),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(
                IpAddr::V4(Ipv4Addr::from_str("176.255.45.12").unwrap()),
                123,
            ),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str("192.168.1.1").unwrap()), 8080),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(
                IpAddr::V4(Ipv4Addr::from_str("10.126.125.98").unwrap()),
                4444,
            ),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str("0.0.0.0").unwrap()), 4444),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(IpAddr::V6(Ipv6Addr::from_str("2001:db8::1").unwrap()), 8080),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(
                IpAddr::V6(Ipv6Addr::from_str("2001:db8:85a3:8d3:1319:8a2e:370:7348").unwrap()),
                443,
            ),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(
                IpAddr::V6(Ipv6Addr::from_str("abcd:ef::42:1").unwrap()),
                8080,
            ),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(
                IpAddr::V6(Ipv6Addr::from_str("::ffff:1.2.3.4").unwrap()),
                4444,
            ),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(IpAddr::V6(Ipv6Addr::from_str("::1").unwrap()), 1234),
            Tcp,
        ),
        NameServerConfig::new(
            SocketAddr::new(IpAddr::V6(Ipv6Addr::from_str("::c0a8:1e02").unwrap()), 8001),
            Tcp,
        ),
    ];

    assert!(config.domain().is_none());
    assert!(config.search().is_empty());

    for item in config.name_servers().iter().zip(expected) {
        assert_eq!(item.0.socket_addr, item.1.socket_addr);
        assert_eq!(item.0.protocol, item.1.protocol);
        assert_eq!(item.0.tls_dns_name, item.1.tls_dns_name);
        assert_eq!(item.0.bind_addr, item.1.bind_addr);
        assert_eq!(item.0.http_endpoint, item.1.http_endpoint);
        assert_eq!(
            item.0.trust_negative_responses,
            item.1.trust_negative_responses
        );
    }
}
