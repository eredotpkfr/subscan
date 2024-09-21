use crate::{
    modules::engines::{bing, duckduckgo, google, yahoo},
    modules::integrations::{alienvault, anubis, bevigil},
    SubscanModule,
};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    /// All `subscan` modules are stores in this in-memory [`Vec`]
    /// as a [`SubscanModule`], all modules must be compatible
    /// with [`SubscanModuleInterface`](crate::interfaces::module::SubscanModuleInterface) trait
    pub static ref ALL_MODULES: Vec<Mutex<SubscanModule>> = vec![
        // Search engines
        SubscanModule::new(google::Google::new()),
        SubscanModule::new(yahoo::Yahoo::new()),
        SubscanModule::new(bing::Bing::new()),
        SubscanModule::new(duckduckgo::DuckDuckGo::new()),
        // API integrations
        SubscanModule::new(alienvault::AlienVault::new()),
        SubscanModule::new(anubis::Anubis::new()),
        SubscanModule::new(bevigil::Bevigil::new()),
    ];
}

/// Module to manage modules that already cached in-memory cache
pub mod modules {
    use crate::{interfaces::requester::RequesterInterface, types::config::RequesterConfig};

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
