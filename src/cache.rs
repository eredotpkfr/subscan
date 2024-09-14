use crate::{
    enums::{RequesterDispatcher, RequesterType},
    interfaces::module::SubscanModuleInterface,
    modules::engines::{bing, duckduckgo, google, yahoo},
    requesters::{chrome, client},
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static! {
    /// All subdomain enumeration modules are stores in this in-memory [`Vec`]
    /// as a [`SubscanModuleInterface`]
    pub static ref ALL_MODULES: Vec<Mutex<Box<dyn SubscanModuleInterface>>> = vec![
        Mutex::new(Box::new(google::Google::new())),
        Mutex::new(Box::new(yahoo::Yahoo::new())),
        Mutex::new(Box::new(bing::Bing::new())),
        Mutex::new(Box::new(duckduckgo::DuckDuckGo::new())),
    ];
    /// All HTTP requester objects are stores in this in-memory [`HashMap`]
    /// as a [`RequesterInterface`](crate::interfaces::requester::RequesterInterface)
    pub static ref ALL_REQUESTERS: HashMap<RequesterType, Mutex<RequesterDispatcher>> =
        HashMap::from([
            (
                RequesterType::ChromeBrowser,
                Mutex::new(RequesterDispatcher::ChromeBrowser(
                    chrome::ChromeBrowser::new()
                ))
            ),
            (
                RequesterType::HTTPClient,
                Mutex::new(RequesterDispatcher::HTTPClient(client::HTTPClient::new()))
            ),
        ]);
}
/// Module to manage in-memory requester cache, basic cache functions
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
    /// [`RequesterInterface`] instance object in [`crate::cache`]
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
