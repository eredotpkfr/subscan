use crate::{
    enums::SubscanModuleDispatcher,
    modules::{
        engines::{bing, duckduckgo, google, yahoo},
        integrations::{
            alienvault, anubis, bevigil, binaryedge, bufferover, builtwith, censys, certspotter,
            chaos, crtsh, digitorus, hackertarget,
        },
    },
};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    /// All `subscan` modules are stores in this in-memory [`Vec`]
    /// as a [`SubscanModule`], all modules must be compatible
    /// with [`SubscanModuleInterface`](crate::interfaces::module::SubscanModuleInterface) trait
    pub static ref ALL_MODULES: Vec<Mutex<SubscanModuleDispatcher>> = vec![
        // Search engines
        Mutex::new(google::Google::dispatcher()),
        Mutex::new(yahoo::Yahoo::dispatcher()),
        Mutex::new(bing::Bing::dispatcher()),
        Mutex::new(duckduckgo::DuckDuckGo::dispatcher()),
        // API integrations
        Mutex::new(alienvault::AlienVault::dispatcher()),
        Mutex::new(anubis::Anubis::dispatcher()),
        Mutex::new(bevigil::Bevigil::dispatcher()),
        Mutex::new(binaryedge::BinaryEdge::dispatcher()),
        Mutex::new(bufferover::BufferOver::dispatcher()),
        Mutex::new(builtwith::BuiltWith::dispatcher()),
        Mutex::new(censys::Censys::dispatcher()),
        Mutex::new(certspotter::CertSpotter::dispatcher()),
        Mutex::new(chaos::Chaos::dispatcher()),
        Mutex::new(crtsh::Crtsh::dispatcher()),
        Mutex::new(digitorus::Digitorus::dispatcher()),
        Mutex::new(hackertarget::HackerTarget::dispatcher())
    ];
}

/// Module to manage modules that already cached in-memory cache
pub mod modules {
    use crate::{
        interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
        types::config::RequesterConfig,
    };

    /// Configure all modules requester objects that has any requester
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use subscan::cache::modules;
    /// use subscan::types::config::RequesterConfig;
    /// use reqwest::header::HeaderMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let new_config = RequesterConfig {
    ///         timeout: Duration::from_secs(120),
    ///         proxy: None,
    ///         headers: HeaderMap::default(),
    ///     };
    ///
    ///     modules::configure_all_requesters(new_config);
    ///
    ///     // configured all modules requester objects
    /// }
    /// ```
    pub async fn configure_all_requesters(config: RequesterConfig) {
        for module in super::ALL_MODULES.iter() {
            let module = module.lock().await;

            if let Some(requester) = module.requester().await {
                requester.lock().await.configure(config.clone()).await;
            }
        }
    }
}
