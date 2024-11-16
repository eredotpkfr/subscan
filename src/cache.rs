use crate::{
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    modules::{
        engines::{bing, duckduckgo, google, yahoo},
        integrations::{
            alienvault, anubis, bevigil, binaryedge, bufferover, builtwith, censys, certspotter,
            chaos, commoncrawl, crtsh, digitorus, dnsdumpsterapi, dnsdumpstercrawler, dnsrepo,
            github, hackertarget, leakix, netcraft, netlas, securitytrails, shodan, sitedossier,
            subdomaincenter, threatcrowd, virustotal, waybackarchive, whoisxmlapi, zoomeye,
        },
        zonetransfer,
    },
    types::{config::requester::RequesterConfig, core::SubscanModule},
};
use lazy_static::lazy_static;

/// Manage cache module
#[derive(Default)]
pub struct CacheManager {}

lazy_static! {
    /// All `Subscan` modules are stores in-memory [`Vec`] as a [`SubscanModule`](crate::types::core::SubscanModule)
    static ref MODULE_CACHE: Vec<SubscanModule> = vec![
        // Search engines
        SubscanModule::from(bing::Bing::dispatcher()),
        SubscanModule::from(duckduckgo::DuckDuckGo::dispatcher()),
        SubscanModule::from(google::Google::dispatcher()),
        SubscanModule::from(yahoo::Yahoo::dispatcher()),
        // Integrations
        SubscanModule::from(alienvault::AlienVault::dispatcher()),
        SubscanModule::from(anubis::Anubis::dispatcher()),
        SubscanModule::from(bevigil::Bevigil::dispatcher()),
        SubscanModule::from(binaryedge::BinaryEdge::dispatcher()),
        SubscanModule::from(bufferover::BufferOver::dispatcher()),
        SubscanModule::from(builtwith::BuiltWith::dispatcher()),
        SubscanModule::from(censys::Censys::dispatcher()),
        SubscanModule::from(certspotter::CertSpotter::dispatcher()),
        SubscanModule::from(chaos::Chaos::dispatcher()),
        SubscanModule::from(commoncrawl::CommonCrawl::dispatcher()),
        SubscanModule::from(crtsh::Crtsh::dispatcher()),
        SubscanModule::from(digitorus::Digitorus::dispatcher()),
        SubscanModule::from(dnsdumpstercrawler::DNSDumpsterCrawler::dispatcher()),
        SubscanModule::from(dnsdumpsterapi::DNSDumpsterAPI::dispatcher()),
        SubscanModule::from(dnsrepo::DnsRepo::dispatcher()),
        SubscanModule::from(github::GitHub::dispatcher()),
        SubscanModule::from(hackertarget::HackerTarget::dispatcher()),
        SubscanModule::from(leakix::Leakix::dispatcher()),
        SubscanModule::from(netcraft::Netcraft::dispatcher()),
        SubscanModule::from(netlas::Netlas::dispatcher()),
        SubscanModule::from(securitytrails::SecurityTrails::dispatcher()),
        SubscanModule::from(shodan::Shodan::dispatcher()),
        SubscanModule::from(sitedossier::Sitedossier::dispatcher()),
        SubscanModule::from(subdomaincenter::SubdomainCenter::dispatcher()),
        SubscanModule::from(threatcrowd::ThreatCrowd::dispatcher()),
        SubscanModule::from(virustotal::VirusTotal::dispatcher()),
        SubscanModule::from(waybackarchive::WaybackArchive::dispatcher()),
        SubscanModule::from(whoisxmlapi::WhoisXMLAPI::dispatcher()),
        SubscanModule::from(zoomeye::ZoomEye::dispatcher()),
        // Others
        SubscanModule::from(zonetransfer::ZoneTransfer::dispatcher())
    ];
}

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
    ///     // do something with module
    /// }
    /// ```
    pub async fn module(&self, name: &str) -> Option<&SubscanModule> {
        for module in self.modules().await.iter() {
            if module.lock().await.name().await == name {
                return Some(module);
            }
        }
        None
    }

    /// Get in-memory modules cache
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::cache::CacheManager;
    ///
    /// #[tokio::main]
    /// async fn main () {
    ///     let manager = CacheManager::default();
    ///     let modules = manager.modules().await;
    ///
    ///     for module in modules.iter() {
    ///         let module = module.lock().await;
    ///
    ///         // do something with module
    ///     }
    /// }
    /// ````
    pub async fn modules(&self) -> &Vec<SubscanModule> {
        &MODULE_CACHE
    }

    /// Configure all modules requester objects that has any requester
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use subscan::cache::CacheManager;
    /// use subscan::types::config::requester::RequesterConfig;
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
        for module in self.modules().await.iter() {
            if let Some(requester) = module.lock().await.requester().await {
                requester.lock().await.configure(config.clone()).await
            }
        }
    }
}
