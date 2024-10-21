use clap::Parser;
use subscan::{cli::Cli, Subscan};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let subscan = Subscan::from(cli);

    println!("{:#?}", subscan.config);

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
