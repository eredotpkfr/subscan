use clap::Parser;
use subscan::{
    cache::{self, ALL_MODULES, ALL_REQUESTERS},
    cli::Cli,
    interfaces::requester::RequesterInterface,
    types::config::RequesterConfig,
};

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
        let mut module = item.lock().await;

        if module.name().await != String::from("DuckDuckGo") {
            continue;
        }

        println!("Running...{}({})", module.name().await, cli.domain.clone());

        module.run(cli.domain.clone()).await;
    }
}
