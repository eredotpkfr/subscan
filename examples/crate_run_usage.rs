use std::env;

use subscan::{types::config::subscan::SubscanConfig, Subscan};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let (module, target) = (&args[1], &args[2]);

    // use default configurations
    let config = SubscanConfig::default();

    let subscan = Subscan::from(config);
    let result = subscan.run(module, target).await;

    for item in result.items {
        println!("{}", item.as_txt())
    }
}
