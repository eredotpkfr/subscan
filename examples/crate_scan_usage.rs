use std::env;

use log::LevelFilter::Debug;
use subscan::{
    enums::cache::CacheFilter::FilterByName,
    types::{config::subscan::SubscanConfig, filters::ModuleNameFilter},
    Subscan,
};

#[tokio::main]
async fn main() {
    let exe = env::current_exe().unwrap();
    let exe_name = exe.file_name().unwrap().to_str();
    let args: Vec<String> = env::args().collect();

    env_logger::builder()
        .filter_module(exe_name.unwrap(), Debug)
        .init();

    // filter modules by name, only runs google and alienvault modules
    let filter = ModuleNameFilter {
        valids: vec!["google".into(), "alienvault".into()],
        invalids: vec![],
    };

    // set module conccurrency to 1
    // set HTTP timeout to 120
    let config = SubscanConfig {
        concurrency: 1,
        timeout: 120,
        filter: FilterByName(filter),
        ..Default::default()
    };

    let subscan = Subscan::from(config);
    let result = subscan.scan(&args[1]).await;

    for item in result.items {
        log::debug!("{}", item.as_txt())
    }
}
