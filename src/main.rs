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
use crate::interfaces::requester::RequesterInterface;
use modules::all::get_all_modules;
use requesters::browser::ChromeBrowser;
use reqwest::Client;
use reqwest::{Method, Url};

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
    let requester = Box::new(ChromeBrowser::new());
    let hrequester = Box::new(Client::new());

    for item in get_all_modules().iter_mut() {
        let _ = item.run(cli.domain.clone(), hrequester.clone()).await;
    }

    //instance.start().await;
}
