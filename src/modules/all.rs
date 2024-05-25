use crate::interfaces::module::SubscanModuleInterface;
use crate::modules::engines::{bing, google, yahoo};

pub fn get_all_modules() -> Vec<Box<dyn SubscanModuleInterface>> {
    vec![
        Box::new(google::Google::new()),
        Box::new(yahoo::Yahoo::new()),
        Box::new(bing::Bing::new()),
    ]
}
