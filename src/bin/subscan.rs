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

    match cli.command {
        Commands::Module(module) => match module.command {
            ModuleSubCommands::List(_) => {}
            ModuleSubCommands::Get(_) => {}
            ModuleSubCommands::Run(args) => {
                println!("{:#?}", subscan.config);
                subscan.run(&args.name, &args.domain).await;
            }
        },
        Commands::Scan(_) => {
            println!("{:#?}", subscan.config);
        }
        Commands::Brute(_) => {}
    }
}
