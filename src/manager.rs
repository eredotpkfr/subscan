use crate::enums::RequesterType;
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use crate::modules::engines::{bing, google, yahoo};
use crate::requesters::browser::ChromeBrowser;
use crate::requesters::client::HTTPClient;
use crate::types::config::RequesterConfig;
use std::collections::HashMap;
use std::sync::OnceLock;

static MODULES: OnceLock<Vec<Box<dyn SubscanModuleInterface>>> = OnceLock::new();
static REQUESTERS: OnceLock<HashMap<RequesterType, Box<dyn RequesterInterface>>> = OnceLock::new();

pub struct Manager {}

impl Manager {
    pub fn modules() -> &'static Vec<Box<dyn SubscanModuleInterface>> {
        Self::register_modules()
    }

    pub fn register_modules() -> &'static Vec<Box<dyn SubscanModuleInterface>> {
        MODULES.get_or_init(|| {
            println!("initialized");
            vec![
                Box::new(google::Google::new()),
                Box::new(yahoo::Yahoo::new()),
                Box::new(bing::Bing::new()),
            ]
        })
    }

    pub fn register_requesters(
        config: RequesterConfig,
    ) -> &'static HashMap<RequesterType, Box<dyn RequesterInterface>> {
        let mut requesters: HashMap<RequesterType, Box<dyn RequesterInterface>> = HashMap::from([
            (
                RequesterType::ChromeBrowser,
                Box::new(ChromeBrowser::new()) as Box<dyn RequesterInterface>,
            ),
            (
                RequesterType::HTTPClient,
                Box::new(HTTPClient::new()) as Box<dyn RequesterInterface>,
            ),
        ]);

        let _ = requesters
            .iter_mut()
            .map(|item| item.1.configure(config.clone()));

        REQUESTERS.get_or_init(|| requesters)
    }
}
