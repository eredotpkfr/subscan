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
                let result = subscan.run(&args.name, &args.domain).await;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
        },
        Commands::Scan(args) => {
            let result = subscan.scan(&args.domain).await;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Commands::Brute(_) => {}
    }
}
