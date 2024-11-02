use clap::Parser;
use subscan::config::{DEFAULT_CONCURRENCY, DEFAULT_HTTP_TIMEOUT, DEFAULT_USER_AGENT};
use subscan::{cli::Cli, enums::output::OutputFormat};

#[tokio::test]
#[should_panic]
async fn scan_parse_error_test() {
    Cli::try_parse_from(vec!["subscan", "scan", "-x"]).unwrap();
}

#[tokio::test]
async fn scan_default_args_test() {
    let args = vec!["subscan", "scan", "-d", "foo.com"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Scan(args) => {
            assert_eq!(args.user_agent, DEFAULT_USER_AGENT);
            assert_eq!(args.proxy, None);
            assert_eq!(args.timeout, DEFAULT_HTTP_TIMEOUT.as_secs());
            assert_eq!(args.concurrency, DEFAULT_CONCURRENCY);
            assert_eq!(args.modules, "*");
            assert_eq!(args.skips, "");
            assert_eq!(args.output, OutputFormat::TXT);
        }
        _ => panic!("Expected Commands::Scan"),
    }
}

#[tokio::test]
async fn scan_args_test() {
    #[rustfmt::skip]
    let args = vec![
        "subscan",
        "scan",
        "-d", "foo.com",
        "--user-agent", "foo",
        "--proxy", "bar",
        "-t", "120",
        "--modules", "google,yahoo",
        "--skips", "commoncrawl",
        "-c", "10",
        "--output", "csv",
    ];

    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Scan(args) => {
            assert_eq!(args.user_agent, "foo");
            assert_eq!(args.proxy.unwrap(), "bar");
            assert_eq!(args.timeout, 120);
            assert_eq!(args.concurrency, 10);
            assert_eq!(args.modules, "google,yahoo");
            assert_eq!(args.skips, "commoncrawl");
            assert_eq!(args.output, OutputFormat::CSV);
        }
        _ => panic!("Expected Commands::Scan"),
    }
}
