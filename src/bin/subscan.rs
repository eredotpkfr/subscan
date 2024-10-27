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

    cli.init().await;
    cli.banner().await;

    match cli.command {
        Commands::Module(module) => match module.command {
            ModuleSubCommands::List(list) => {
                list.as_table(subscan.modules().await).await;
            }
            ModuleSubCommands::Get(get) => {
                get.as_table(subscan.module(&get.name).await).await;
            }
            ModuleSubCommands::Run(args) => {
                subscan.run(&args.name, &args.domain).await;
            }
        },
        Commands::Scan(args) => {
            for sub in subscan.scan(&args.domain).await {
                log::debug!("{}", sub);
            }
        }
        Commands::Brute(_) => {}
    }
}
