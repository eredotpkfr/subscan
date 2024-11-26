use std::env;

use subscan::{
    enums::cache::CacheFilter::FilterByName,
    types::{config::subscan::SubscanConfig, filters::ModuleNameFilter},
    Subscan,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // filter modules by name, only runs google and alienvault modules
    let filter = ModuleNameFilter {
        valids: vec!["google".into(), "alienvault".into()],
        invalids: vec![],
    };

    // set module conccurrency to 1
    // set http timeout to 120
    let config = SubscanConfig {
        concurrency: 1,
        timeout: 120,
        filter: FilterByName(filter),
        ..Default::default()
    };

    let subscan = Subscan::from(config);
    let result = subscan.scan(&args[1]).await;

    for item in result.items {
        println!("{}", item.as_txt())
    }
}
