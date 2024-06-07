use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use crate::modules::engines::{bing, google, yahoo};
use crate::requesters::{browser, client};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref ALL_MODULES: Vec<Mutex<Box<dyn SubscanModuleInterface>>> = vec![
        Mutex::new(Box::new(google::Google::new())),
        Mutex::new(Box::new(yahoo::Yahoo::new())),
        Mutex::new(Box::new(bing::Bing::new())),
    ];
    pub static ref ALL_REQUESTERS: Vec<Mutex<Box<dyn RequesterInterface>>> = vec![
        Mutex::new(Box::new(browser::ChromeBrowser::new())),
        Mutex::new(Box::new(client::HTTPClient::new())),
    ];
}
