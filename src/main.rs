mod engines;
mod integrations;
mod utils;

use clap::Parser;
//use engines::google::Google;
//use engines::yahoo::Yahoo;
//use engines::bing::Bing;
//use integrations::alienvault::AlienVault;
use integrations::anubis::Anubis;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target domain address to be enumerated
    domain: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    //let instance = Google::new(cli.domain).await;
    //let instance = Yahoo::new(cli.domain).await;
    //let instance = Bing::new(cli.domain).await;
    //let instance = AlienVault::new(cli.domain).await;
    let instance = Anubis::new(cli.domain).await;

    instance.start().await;
}
