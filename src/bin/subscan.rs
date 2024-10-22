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
                println!("{} {}", args.name, args.domain);
            }
        },
        Commands::Scan(_) => {
            println!("{:#?}", subscan.config);
        }
        Commands::Brute(_) => {}
    }

    // cache::modules::configure_all_requesters(config).await;

    // for item in ALL_MODULES.iter() {
    //     let mut module = item.lock().await;
    //     let requester = module.requester().await.unwrap();

    //     if module.name().await != "dnsrepo" {
    //         continue;
    //     }

    //     println!(
    //         "{:#?} {:p}",
    //         requester.lock().await.config().await,
    //         requester,
    //     );

    //     println!("Running...{}({})", module.name().await, cli.domain.clone());

    //     let res = module.run(&cli.domain).await;

    //     println!("{:#?}\nTotal: {}", res, res.len());
    // }
}
