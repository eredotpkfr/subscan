use std::io;

use clap::Parser;
use subscan::{
    cli::{
        commands::{module::ModuleSubCommands, Commands},
        Cli,
    },
    Subscan,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let subscan = Subscan::from(cli.clone());
    let out = &mut io::stdout();

    cli.init().await;
    cli.banner().await;

    match cli.command {
        Commands::Module(module) => match module.command {
            ModuleSubCommands::List(list) => {
                list.as_table(subscan.modules().await, out).await;
            }
            ModuleSubCommands::Get(get) => {
                get.as_table(subscan.module(&get.name).await, out).await;
            }
            ModuleSubCommands::Run(args) => {
                subscan.run(&args.name, &args.domain).await.save(&args.output).await
            }
        },
        Commands::Scan(args) => {
            subscan.scan(&args.domain).await.save(&args.output).await;
        }
        Commands::Brute(args) => {
            if args.stream_to_txt.is_some() {
                subscan.brute(&args.domain).await;
            } else {
                subscan.brute(&args.domain).await.save(&args.output).await;
            }
        }
    }
}
