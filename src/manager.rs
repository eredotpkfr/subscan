use crate::enums::{RequesterDispatcher, RequesterType};
use crate::interfaces::{module::SubscanModuleInterface, requester::RequesterInterface};
use crate::modules::engines::{bing, google, yahoo};
use crate::requesters::{chrome, client};
use futures::{stream, StreamExt};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref ALL_MODULES: Vec<Mutex<Box<dyn SubscanModuleInterface>>> = vec![
        Mutex::new(Box::new(google::Google::new())),
        Mutex::new(Box::new(yahoo::Yahoo::new())),
        Mutex::new(Box::new(bing::Bing::new())),
    ];
    pub static ref ALL_REQUESTERS: Vec<Mutex<RequesterDispatcher>> = vec![
        Mutex::new(RequesterDispatcher::ChromeBrowser(
            chrome::ChromeBrowser::new()
        )),
        Mutex::new(RequesterDispatcher::HTTPClient(client::HTTPClient::new())),
    ];
}

pub async fn get_requester_by_type<'a>(rtype: RequesterType) -> &'a Mutex<RequesterDispatcher> {
    let filtered = stream::iter(ALL_REQUESTERS.iter())
        .filter(|item| async { item.lock().unwrap().r#type().await == rtype })
        .collect::<Vec<&Mutex<RequesterDispatcher>>>()
        .await;

    filtered.first().expect("Requester not found!")
}
