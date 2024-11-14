use tokio::sync::Notify;

use super::{
    constants::{LOCAL_HOST, TEST_DOMAIN},
    dns::MockDNSServer,
};
use std::{net::TcpListener, sync::Arc};

pub fn get_random_port() -> u16 {
    TcpListener::bind(format!("{LOCAL_HOST}:0"))
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

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
