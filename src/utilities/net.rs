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
