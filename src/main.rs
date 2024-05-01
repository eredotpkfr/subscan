mod engines;

use clap::Parser;
use engines::google::Google;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target domain address to be enumerated
    domain: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let google = Google::new(cli.domain).await;

    google.start().await;
}
