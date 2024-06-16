mod cache;
mod enums;
mod extractors;
mod integrations;
mod interfaces;
mod modules;
mod requesters;
mod types;
mod utils;

use clap::Parser;
//use integrations::alienvault::AlienVault;
//use integrations::anubis::Anubis;
use crate::{
    cache::{ALL_MODULES, ALL_REQUESTERS},
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    types::config::RequesterConfig,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target domain address to be enumerated
    #[arg(short, long)]
    domain: String,
    /// User-Agent header value for HTTP requests
    #[arg(
        short,
        long,
        default_value = "Mozilla/5.0 (Macintosh; \
            Intel Mac OS X 10_15_7) AppleWebKit/537.36  \
            (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
    )]
    user_agent: String,
    /// HTTP timeout value as a seconds
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,
    /// HTTP proxy
    #[arg(short, long, default_value = None)]
    proxy: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = RequesterConfig::from_cli(&cli);

    cache::requesters::configure_all(config).await;

    for requester in ALL_REQUESTERS.values() {
        println!(
            "{:#?} {:p}",
            requester.lock().await.config().await,
            requester
        );
    }

    for item in ALL_MODULES.iter() {
        let module = item.lock().await;

        println!("Running...{}({})", module.name().await, cli.domain.clone());

        // module.run(cli.domain.clone()).await;
    }
    //instance.start().await;
}
