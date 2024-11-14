use std::sync::Arc;

use crate::common::{constants::TEST_DOMAIN, utils::current_thread_hex};
use reqwest::Url;
use subscan::enums::dispatchers::SubscanModuleDispatcher;
use tokio::sync::Notify;

use super::dns::MockDNSServer;

pub async fn spawn_mock_dns_server() -> MockDNSServer {
    let notify_one = Arc::new(Notify::new());
    let notift_two = notify_one.clone();

    let server = MockDNSServer::new(TEST_DOMAIN);
    let cloned = server.clone();

    tokio::spawn(async move {
        notift_two.notify_one();
        cloned.start().await;
    });

    notify_one.notified().await;
    server
}

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

pub fn wrap_module_name(dispatcher: &mut SubscanModuleDispatcher, name: String) {
    match dispatcher {
        SubscanModuleDispatcher::GenericIntegrationModule(module) => {
            module.name = current_thread_hex()
        }
        SubscanModuleDispatcher::GenericSearchEngineModule(module) => {
            module.name = current_thread_hex()
        }
        SubscanModuleDispatcher::GitHub(module) => module.name = name,
        _ => {}
    }
}
