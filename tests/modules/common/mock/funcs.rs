use crate::common::utils::current_thread_hex;
use reqwest::Url;
use subscan::enums::dispatchers::SubscanModuleDispatcher;

pub fn wrap_url_with_mock_func(url: &str) -> Box<dyn Fn(&str) -> String + Sync + Send> {
    let url: Url = url.parse().unwrap();

    Box::new(move |_| url.to_string().clone())
}

pub fn wrap_module_url(dispatcher: &mut SubscanModuleDispatcher, url: &str) {
    match dispatcher {
        SubscanModuleDispatcher::GenericSearchEngineModule(module) => {
            module.url = url.parse().unwrap()
        }
        SubscanModuleDispatcher::GenericIntegrationModule(module) => {
            module.funcs.url = wrap_url_with_mock_func(url)
        }
        SubscanModuleDispatcher::CommonCrawl(module) => module.url = url.parse().unwrap(),
        SubscanModuleDispatcher::DnsDumpster(module) => module.url = url.parse().unwrap(),
        SubscanModuleDispatcher::GitHub(module) => module.url = url.parse().unwrap(),
        SubscanModuleDispatcher::Netlas(module) => module.url = url.parse().unwrap(),
        SubscanModuleDispatcher::WaybackArchive(module) => module.url = url.parse().unwrap(),
        _ => {}
    }
}

pub fn wrap_module_name(dispatcher: &mut SubscanModuleDispatcher) {
    match dispatcher {
        SubscanModuleDispatcher::GenericIntegrationModule(module) => {
            module.name = current_thread_hex()
        }
        SubscanModuleDispatcher::GenericSearchEngineModule(module) => {
            module.name = current_thread_hex()
        }
        SubscanModuleDispatcher::GitHub(module) => {
            module.name = format!("{}_{}", module.name, current_thread_hex())
        }
        _ => {}
    }
}
