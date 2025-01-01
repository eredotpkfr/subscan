use std::path::PathBuf;

use clap::Parser;
use subscan::{
    cli::Cli,
    constants::{DEFAULT_RESOLVER_CONCURRENCY, DEFAULT_RESOLVER_TIMEOUT},
    enums::output::OutputFormat,
};

#[tokio::test]
#[should_panic]
async fn brute_parse_error_test() {
    Cli::try_parse_from(vec!["subscan", "brute", "-x"]).unwrap();
}

#[tokio::test]
async fn brute_default_args_test() {
    let args = vec!["subscan", "brute", "-d", "foo.com", "-w", "wordlist.txt"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Brute(args) => {
            assert!(!args.print);

            assert_eq!(args.domain, "foo.com");
            assert_eq!(args.output, OutputFormat::JSON);
            assert_eq!(args.resolver_concurrency, DEFAULT_RESOLVER_CONCURRENCY);
            assert_eq!(
                args.resolver_timeout,
                DEFAULT_RESOLVER_TIMEOUT.as_millis() as u64
            );
        }
        _ => panic!("Expected Commands::Brute"),
    }
}

#[tokio::test]
async fn brute_args_test() {
    #[rustfmt::skip]
    let args = vec![
        "subscan",
        "brute",
        "-d", "foo.com",
        "--wordlist", "wordlist.txt",
        "--output", "csv",
        "--resolver-concurrency", "100",
        "--resolver-timeout", "10",
        "--print"
    ];

    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Brute(args) => {
            assert!(args.print);

            assert_eq!(args.domain, "foo.com");
            assert_eq!(args.wordlist, PathBuf::from("wordlist.txt"));
            assert_eq!(args.output, OutputFormat::CSV);
            assert_eq!(args.resolver_concurrency, 100);
            assert_eq!(args.resolver_timeout, 10);
        }
        _ => panic!("Expected Commands::Brute"),
    }
}
