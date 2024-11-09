use crate::common::constants::LOCAL_HOST;
use serde_json::Value;
use std::{
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    thread,
};

pub fn stubs_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/stubs")
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

pub fn read_stub(path: &str) -> Value {
    let file_path = stubs_path().join(path);
    let content = fs::read_to_string(file_path).unwrap();

    serde_json::from_str(&content).unwrap()
}
