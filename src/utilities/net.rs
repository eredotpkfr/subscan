use std::net::IpAddr;

use hickory_resolver::{
    config::{NameServerConfig, NameServerConfigGroup, Protocol::Tcp},
    system_conf,
};

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

    if let Ok((config, _)) = system_conf::read_system_conf() {
        let sys_ns = config.name_servers();

        if !sys_ns.is_empty() {
            return sys_ns.iter().find(tcp).cloned();
        }
    }

    NameServerConfigGroup::cloudflare()
        .iter()
        .find(tcp)
        .cloned()
}

/// Lookup IP address of any hostname
///
/// # Examples
///
/// ```no_run
/// use subscan::utilities::net::lookup_host;
///
/// #[tokio::main]
/// async fn main() {
///     let ip = lookup_host("foo.com").await;
///     // do something with ip
/// }
/// ```
pub async fn lookup_host(domain: &str) -> Option<IpAddr> {
    dns_lookup::lookup_host(domain).ok()?.first().cloned()
}
