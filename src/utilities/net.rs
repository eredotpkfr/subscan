use std::{
    fs::read_to_string,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use hickory_client::proto::xfer::Protocol::Tcp;
use hickory_resolver::{
    config::{NameServerConfig, NameServerConfigGroup, ResolverConfig as HickoryResolverConfig},
    system_conf, ResolveError,
};
use regex::Regex;

use crate::constants::{RL_IPV4_PATTERN, RL_IPV6_PATTERN, RL_PORT_PATTERN};

const INVALID_RESOLVER_LIST_FILE_FORMAT_ERR_MSG: &str = "Invalid resolver list file format!";
const RESOLVER_LIST_FILE_READ_ERR_MSG: &str = "Cannot read resolver list file!";

/// Returns default name servers from system configurations, if not fetch
/// system confs tries to get cloudflare name servers as a default
///
/// # Examples
///
/// ```
/// use subscan::utilities::net::get_default_ns;
///
/// assert!(get_default_ns().is_some());
/// ```
pub fn get_default_ns() -> Option<NameServerConfig> {
    let tcp = |ns: &&NameServerConfig| ns.protocol == Tcp;
    let default = NameServerConfigGroup::cloudflare().iter().find(tcp).cloned();

    read_system_ns_conf().map_or(default, |config| {
        config.name_servers().iter().find(tcp).cloned()
    })
}

/// Try to read system name server configurations, mostly /etc/resolv.conf
/// file on linux like systems
///
/// # Examples
///
/// ```no_run
/// use subscan::utilities::net::read_system_ns_conf;
///
/// assert!(read_system_ns_conf().is_ok());
/// ```
pub fn read_system_ns_conf() -> Result<HickoryResolverConfig, ResolveError> {
    Ok(system_conf::read_system_conf()?.0)
}

pub fn read_resolver_list_file(path: PathBuf) -> HickoryResolverConfig {
    let ipv4_pattern = Regex::new(&format!("{RL_IPV4_PATTERN}:{RL_PORT_PATTERN}$")).unwrap();
    let ipv6_pattern = Regex::new(&format!("{RL_IPV6_PATTERN}:{RL_PORT_PATTERN}$")).unwrap();

    let content = read_to_string(path).expect(RESOLVER_LIST_FILE_READ_ERR_MSG);
    let mut config = HickoryResolverConfig::new();

    for line in content.lines() {
        if let Some(caps) = ipv4_pattern.captures(line) {
            let ip = caps.name("ip").expect(INVALID_RESOLVER_LIST_FILE_FORMAT_ERR_MSG);
            let port = caps.name("port").expect(INVALID_RESOLVER_LIST_FILE_FORMAT_ERR_MSG);

            if let Ok(ipv4) = Ipv4Addr::from_str(ip.as_str()) {
                let socket = SocketAddr::new(IpAddr::V4(ipv4), port.as_str().parse().unwrap());
                let ns = NameServerConfig::new(socket, Tcp);

                config.add_name_server(ns);
            }
        }

        if let Some(caps) = ipv6_pattern.captures(line) {
            let ip = caps.name("ip").expect(INVALID_RESOLVER_LIST_FILE_FORMAT_ERR_MSG);
            let port = caps.name("port").expect(INVALID_RESOLVER_LIST_FILE_FORMAT_ERR_MSG);

            if let Ok(ipv6) = Ipv6Addr::from_str(ip.as_str()) {
                let socket = SocketAddr::new(IpAddr::V6(ipv6), port.as_str().parse().unwrap());
                let ns = NameServerConfig::new(socket, Tcp);

                config.add_name_server(ns);
            }
        }
    }

    config
}
