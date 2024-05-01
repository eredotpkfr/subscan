use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Target domain address to be enumerated
    domain: String,

    /// Set HTTP proxy for requests
    #[arg(short = 'p', long)]
    http_proxy: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", cli.domain)
    //    println!("{:#?}", cli);
}
