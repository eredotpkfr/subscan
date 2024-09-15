use crate::{
    enums::{RequesterDispatcher, RequesterType},
    modules::engines::{bing, duckduckgo, google, yahoo},
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
    SubscanModule,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static! {
    /// All `subscan` modules are stores in this in-memory [`Vec`]
    /// as a [`SubscanModule`], all modules must be compatible
    /// with [`SubscanModuleInterface`](crate::interfaces::module::SubscanModuleInterface) trait
    pub static ref ALL_MODULES: Vec<Mutex<SubscanModule>> = vec![
        SubscanModule::new(google::Google::new()),
        SubscanModule::new(yahoo::Yahoo::new()),
        SubscanModule::new(bing::Bing::new()),
        SubscanModule::new(duckduckgo::DuckDuckGo::new()),
    ];
    /// All HTTP requester objects are stores in this in-memory [`HashMap`]
    /// as a [`RequesterInterface`](crate::interfaces::requester::RequesterInterface)
    pub static ref ALL_REQUESTERS: HashMap<RequesterType, Mutex<RequesterDispatcher>> =
        HashMap::from([
            (
                RequesterType::ChromeBrowser,
                Mutex::new(ChromeBrowser::new().try_into().unwrap())
            ),
            (
                RequesterType::HTTPClient,
                Mutex::new(HTTPClient::default().try_into().unwrap())
            ),
        ]);
}

/// Module to manage in-memory requester cache. Basic cache functions
/// are listed in here
pub mod requesters {
    use tokio::sync::Mutex;

    use super::ALL_REQUESTERS;
    use crate::{
        enums::{RequesterDispatcher, RequesterType},
        interfaces::requester::RequesterInterface,
        types::config::RequesterConfig,
    };

    /// Enumerates all pre-defined HTTP requesters on in-memory [`std::collections::HashMap`]
    /// and returns [`RequesterDispatcher`] by given [`RequesterType`]
    ///
    /// # Panics
    ///
    /// When the given [`RequesterType`] did not mapped any
    /// [`RequesterDispatcher`] instance object in [`struct@crate::cache::ALL_REQUESTERS`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::cache;
    /// use subscan::enums::RequesterType;
    /// use subscan::interfaces::requester::RequesterInterface;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let rtype = RequesterType::HTTPClient;
    ///     let requester = cache::requesters::get_by_type(&rtype).lock().await;
    ///
    ///     // do something with requester instance
    /// }
    /// ```
    pub fn get_by_type(rtype: &RequesterType) -> &Mutex<RequesterDispatcher> {
        ALL_REQUESTERS.get(rtype).expect("Requester not found!")
    }

    /// Configure all pre-defined HTTP requester instances with using
    /// single stupid function according to given any [`RequesterConfig`]
    /// object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use reqwest::header::HeaderMap;
    /// use subscan::cache;
    /// use subscan::types::config::RequesterConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let new_config = RequesterConfig {
    ///         timeout: Duration::from_secs(120),
    ///         headers: HeaderMap::default(),
    ///         proxy: None,
    ///     };
    ///
    ///     cache::requesters::configure_all(new_config).await;
    /// }
    /// ```
    pub async fn configure_all(config: RequesterConfig) {
        for requester in ALL_REQUESTERS.values() {
            requester.lock().await.configure(config.clone()).await
        }
    }
}
