use crate::interfaces::module::SubscanModuleInterface;
use crate::modules::google::Google;

pub fn get_all_modules() -> Vec<Box<dyn SubscanModuleInterface>> {
    vec![Google::new()]
}
