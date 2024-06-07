mod enums;
mod extractors;
mod integrations;
mod interfaces;
mod manager;
mod modules;
mod requesters;
mod types;
mod utils;

use clap::Parser;
//use integrations::alienvault::AlienVault;
//use integrations::anubis::Anubis;
use crate::manager::ALL_MODULES;
use crate::types::query::SearchQueryParam;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target domain address to be enumerated
    domain: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    //let instance = AlienVault::new(cli.domain).await;
    //let instance = Anubis::new(cli.domain).await;

    // for item in get_all_modules().iter_mut() {
    //     let _ = item.run(cli.domain.clone()).await;
    // }

    for item in ALL_MODULES.iter() {
        let mut module = item.lock().unwrap();

        println!("Running...{}", module.name().await);

        module.run(cli.domain.clone()).await;
    }

    //instance.start().await;
}
