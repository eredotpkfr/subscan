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
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use crate::manager::get_requester_by_type;
use crate::manager::{ALL_MODULES, ALL_REQUESTERS};
use crate::types::{config::RequesterConfig, query::SearchQueryParam};

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

    let config: RequesterConfig = RequesterConfig::from_cli(&cli);

    for requester in ALL_REQUESTERS.iter() {
        let _ = requester.lock().unwrap().configure(config.clone()).await;
    }

    // let client = ALL_REQUESTERS[ALL_REQUESTERS.len() - 1].lock().unwrap();
    // println!("{:#?}", client.config().await);

    //let instance = AlienVault::new(cli.domain).await;
    //let instance = Anubis::new(cli.domain).await;

    // for item in get_all_modules().iter_mut() {
    //     let _ = item.run(cli.domain.clone()).await;
    // }

    for requester in ALL_REQUESTERS.iter() {
        println!("{:#?}", requester.lock().unwrap().r#type().await);
    }

    println!(
        "Getted target: {:#?}",
        get_requester_by_type(enums::RequesterType::ChromeBrowser)
            .await
            .lock()
            .unwrap()
            .r#type()
            .await
    );

    for item in ALL_MODULES.iter() {
        let module = item.lock().unwrap();

        println!("Running...{}({})", module.name().await, cli.domain.clone());

        // module.run(domain.clone()).await;
    }
    //instance.start().await;
}
