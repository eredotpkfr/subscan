use std::{
    collections::BTreeSet,
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    thread,
};

use serde_json::Value;
use subscan::{
    enums::{
        content::Content,
        dispatchers::SubscanModuleDispatcher,
        result::{OptionalSubscanModuleResult, SubscanModuleResult},
    },
    interfaces::module::SubscanModuleInterface,
    types::{
        core::{Subdomain, UnboundedFlumeChannel},
        result::status::SubscanModuleStatus,
    },
};

use crate::common::constants::{LOCAL_HOST, READ_ERROR};

pub async fn run_module(
    mut module: SubscanModuleDispatcher,
    domain: &str,
) -> (BTreeSet<Subdomain>, SubscanModuleStatus) {
    let mut results = BTreeSet::new();
    let mut statuses = Vec::new();

    let channel: UnboundedFlumeChannel<OptionalSubscanModuleResult> = flume::unbounded().into();

    module.run(domain, channel.tx).await;

    while let Ok(msg) = channel.rx.recv() {
        if let Some(result) = msg.as_ref() {
            match result {
                SubscanModuleResult::SubscanModuleResultItem(item) => {
                    assert_eq!(
                        item.module,
                        module.name().await,
                        "SubscanModuleResultItem has incorrect module name"
                    );
                    results.insert(item.subdomain.clone());
                }
                SubscanModuleResult::SubscanModuleStatusItem(item) => {
                    assert_eq!(
                        item.module,
                        module.name().await,
                        "SubscanModuleResultItem has incorrect module name"
                    );
                    statuses.push(item.status.clone());
                }
            }
        } else {
            break;
        }
    }

    assert_eq!(
        statuses.len(),
        1,
        "The module only has to send one status update"
    );

    (results, statuses.first().unwrap().clone())
}

pub fn stubs_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/stubs")
}

pub fn read_stub(path: &str) -> Value {
    let file_path = stubs_path().join(path);
    let content = fs::read_to_string(file_path).unwrap();

    serde_json::from_str(&content).unwrap()
}

pub fn testdata_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("testing/testdata")
}

pub fn read_testdata(path: &str) -> Content {
    Content::String(fs::read_to_string(testdata_path().join(path)).expect(READ_ERROR))
}

pub fn get_random_port() -> u16 {
    TcpListener::bind(format!("{LOCAL_HOST}:0"))
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

pub fn md5_hex(target: String) -> String {
    format!("{:x}", md5::compute(target))
}

pub fn current_thread_hex() -> String {
    md5_hex(thread::current().name().unwrap().to_uppercase())
}

pub fn fix_new_lines(input: &str) -> String {
    input.replace("\r\n", "\n")
}
