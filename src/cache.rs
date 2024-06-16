use crate::{
    enums::{RequesterDispatcher, RequesterType},
    interfaces::module::SubscanModuleInterface,
    modules::engines::{bing, google, yahoo},
    requesters::{chrome, client},
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref ALL_MODULES: Vec<Mutex<Box<dyn SubscanModuleInterface>>> = vec![
        Mutex::new(Box::new(google::Google::new())),
        Mutex::new(Box::new(yahoo::Yahoo::new())),
        Mutex::new(Box::new(bing::Bing::new())),
    ];
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

pub mod requesters {
    use tokio::sync::Mutex;

    use crate::interfaces::requester::RequesterInterface;
    use crate::types::config::RequesterConfig;

    pub fn get_by_type(rtype: &super::RequesterType) -> &Mutex<super::RequesterDispatcher> {
        super::ALL_REQUESTERS
            .get(rtype)
            .expect("Requester not found!")
    }

    pub async fn configure_all(config: RequesterConfig) {
        for requester in super::ALL_REQUESTERS.values() {
            requester.lock().await.configure(config.clone()).await
        }
    }
}
