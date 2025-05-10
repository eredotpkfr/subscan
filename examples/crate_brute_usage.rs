use std::{env, io::Write};

use log::LevelFilter::Debug;
use subscan::{types::config::subscan::SubscanConfig, Subscan};
use tempfile::NamedTempFile;

#[tokio::main]
async fn main() {
    let exe = env::current_exe().unwrap();
    let exe_name = exe.file_name().unwrap().to_str();
    let args: Vec<String> = env::args().collect();

    env_logger::builder().filter_module(exe_name.unwrap(), Debug).init();

    let mut wordlist = NamedTempFile::new().unwrap();

    writeln!(wordlist, "api").unwrap();
    writeln!(wordlist, "app").unwrap();
    writeln!(wordlist, "test").unwrap();

    let config = SubscanConfig {
        wordlist: Some(wordlist.path().to_path_buf()),
        ..Default::default()
    };

    let subscan = Subscan::from(config);
    let result = subscan.brute(&args[1]).await;

    for item in result.items {
        log::debug!("{}", item.as_txt())
    }
}
