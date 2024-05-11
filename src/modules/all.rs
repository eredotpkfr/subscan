use crate::interfaces::module::SubscanModuleInterface;
use crate::modules::engines::bing::Bing;
use crate::modules::engines::google::Google;
use crate::modules::engines::yahoo::Yahoo;

pub fn get_all_modules() -> Vec<Box<dyn SubscanModuleInterface>> {
    vec![Google::new(), Yahoo::new(), Bing::new()]
}
