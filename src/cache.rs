use crate::{
    enums::SubscanModuleDispatcher,
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    modules::{
        engines::{bing, duckduckgo, google, yahoo},
        integrations::{
            alienvault, anubis, bevigil, binaryedge, bufferover, builtwith, censys, certspotter,
            chaos, commoncrawl, crtsh, digitorus, dnsdumpster, dnsrepo, github, hackertarget,
            leakix, netlas, securitytrails, shodan, sitedossier, subdomaincenter, threatcrowd,
            virustotal, waybackarchive, whoisxmlapi, zoomeye,
        },
    },
    types::config::RequesterConfig,
};
use lazy_static::lazy_static;
use std::slice::Iter;
use tokio::sync::Mutex;

lazy_static! {
    /// All `Subscan` modules are stores in this in-memory [`Vec`] as a [`SubscanModuleDispatcher`]
    #[derive(Default)]
    pub static ref ALL_MODULES: Vec<Mutex<SubscanModuleDispatcher>> = vec![
        // Search engines
        Mutex::new(bing::Bing::dispatcher()),
        Mutex::new(duckduckgo::DuckDuckGo::dispatcher()),
        Mutex::new(google::Google::dispatcher()),
        Mutex::new(yahoo::Yahoo::dispatcher()),
        // Integrations
        Mutex::new(alienvault::AlienVault::dispatcher()),
        Mutex::new(anubis::Anubis::dispatcher()),
        Mutex::new(bevigil::Bevigil::dispatcher()),
        Mutex::new(binaryedge::BinaryEdge::dispatcher()),
        Mutex::new(bufferover::BufferOver::dispatcher()),
        Mutex::new(builtwith::BuiltWith::dispatcher()),
        Mutex::new(censys::Censys::dispatcher()),
        Mutex::new(certspotter::CertSpotter::dispatcher()),
        Mutex::new(chaos::Chaos::dispatcher()),
        Mutex::new(commoncrawl::CommonCrawl::dispatcher()),
        Mutex::new(crtsh::Crtsh::dispatcher()),
        Mutex::new(digitorus::Digitorus::dispatcher()),
        Mutex::new(dnsdumpster::DnsDumpster::dispatcher()),
        Mutex::new(dnsrepo::DnsRepo::dispatcher()),
        Mutex::new(github::GitHub::dispatcher()),
        Mutex::new(hackertarget::HackerTarget::dispatcher()),
        Mutex::new(leakix::Leakix::dispatcher()),
        Mutex::new(netlas::Netlas::dispatcher()),
        Mutex::new(securitytrails::SecurityTrails::dispatcher()),
        Mutex::new(shodan::Shodan::dispatcher()),
        Mutex::new(sitedossier::Sitedossier::dispatcher()),
        Mutex::new(subdomaincenter::SubdomainCenter::dispatcher()),
        Mutex::new(threatcrowd::ThreatCrowd::dispatcher()),
        Mutex::new(virustotal::VirusTotal::dispatcher()),
        Mutex::new(waybackarchive::WaybackArchive::dispatcher()),
        Mutex::new(whoisxmlapi::WhoisXMLAPI::dispatcher()),
        Mutex::new(zoomeye::ZoomEye::dispatcher()),
    ];
}

#[derive(Default)]
pub struct CacheManager(ALL_MODULES);

impl CacheManager {
    /// Get module by name
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::cache::CacheManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let manager = CacheManager::default();
    ///     let google = manager.module("google").await;
    ///
    ///     // Do something with module
    /// }
    /// ```
    pub async fn module(&self, name: &str) -> Option<&Mutex<SubscanModuleDispatcher>> {
        for module in self.iter().await {
            if module.lock().await.name().await == name {
                return Some(module);
            }
        }

        None
    }

    /// Iterate over cached modules
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::cache::CacheManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let manager = CacheManager::default();
    ///
    ///     for module in manager.iter().await {
    ///         // Iterate over modules
    ///     }
    /// }
    /// ```
    pub async fn iter(&self) -> Iter<Mutex<SubscanModuleDispatcher>> {
        self.0.iter()
    }

    /// Configure all modules requester objects that has any requester
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use subscan::cache::CacheManager;
    /// use subscan::types::config::RequesterConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let new_config = RequesterConfig {
    ///         timeout: Duration::from_secs(120),
    ///         ..Default::default()
    ///     };
    ///
    ///     let manager = CacheManager::default();
    ///
    ///     manager.configure(new_config).await;
    ///
    ///     // configured all modules requester objects
    /// }
    /// ```
    pub async fn configure(&self, config: RequesterConfig) {
        for module in self.iter().await {
            if let Some(requester) = module.lock().await.requester().await {
                requester.lock().await.configure(config.clone()).await
            }
        }
    }
}
