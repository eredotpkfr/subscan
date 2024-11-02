use clap::Parser;
use subscan::cli::Cli;

#[tokio::test]
#[should_panic]
async fn cli_parse_error_test() {
    Cli::try_parse_from(vec!["subscan", "-x"]).unwrap();
}

#[tokio::test]
async fn verbosity_test() {
    let args = vec!["subscan", "scan", "-d", "foo.com", "-qq"];
    let cli = Cli::try_parse_from(args).unwrap();

    cli.init().await;
    cli.banner().await;

    assert_eq!(cli.verbose.is_present(), true);
    assert_eq!(cli.verbose.to_string(), "1");
}
