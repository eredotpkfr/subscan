use std::env;

use log::LevelFilter::Debug;
use subscan::{types::config::subscan::SubscanConfig, Subscan};

#[tokio::main]
async fn main() {
    let exe = env::current_exe().unwrap();
    let exe_name = exe.file_name().unwrap().to_str();

    let args: Vec<String> = env::args().collect();
    let (module, target) = (&args[1], &args[2]);

    env_logger::builder()
        .filter_module(exe_name.unwrap(), Debug)
        .init();

    // use default configurations
    let config = SubscanConfig::default();

    let subscan = Subscan::from(config);
    let result = subscan.run(module, target).await;

    for item in result.items {
        log::debug!("{}", item.as_txt())
    }
}
