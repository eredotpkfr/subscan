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
use crate::manager::Manager;

use crate::types::query::SearchQueryParam;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target domain address to be enumerated
    domain: String,
}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();

    //let instance = AlienVault::new(cli.domain).await;
    //let instance = Anubis::new(cli.domain).await;

    // for item in get_all_modules().iter_mut() {
    //     let _ = item.run(cli.domain.clone()).await;
    // }

    for module in Manager::register_modules() {
        println!("{}", module.name().await);
    }

    for module in Manager::modules() {
        println!("{}", module.name().await);
    }

    for module in Manager::modules() {
        println!("{}", module.name().await);
    }

    //instance.start().await;
}
