use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use subscan::{
    cli::Cli,
    constants::{
        DEFAULT_HTTP_TIMEOUT, DEFAULT_RESOLVER_CONCURRENCY, DEFAULT_RESOLVER_TIMEOUT,
        DEFAULT_USER_AGENT,
    },
};

#[tokio::test]
#[should_panic]
async fn module_run_parse_error_test() {
    Cli::try_parse_from(vec!["subscan", "module", "run", "-x"]).unwrap();
}

#[tokio::test]
async fn module_run_default_args_test() {
    let args = vec!["subscan", "module", "run", "foo", "-d", "bar.com"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Module(sub) => match sub.command {
            subscan::cli::commands::module::ModuleSubCommands::Run(args) => {
                assert!(!args.resolver_disabled);

                assert_eq!(args.domain, "bar.com");
                assert_eq!(args.name, "foo");
                assert_eq!(args.user_agent, DEFAULT_USER_AGENT);
                assert_eq!(args.proxy, None);
                assert_eq!(args.http_timeout, DEFAULT_HTTP_TIMEOUT.as_secs());
                assert_eq!(args.resolver_concurrency, DEFAULT_RESOLVER_CONCURRENCY);
                assert_eq!(
                    args.resolver_timeout,
                    DEFAULT_RESOLVER_TIMEOUT.as_millis() as u64
                );
            }
            _ => panic!("Expected ModuleSubCommands::Run"),
        },
        _ => panic!("Expected Commands::Module"),
    }
}

#[tokio::test]
async fn module_run_args_test() {
    #[rustfmt::skip]
    let args = vec![
        "subscan",
        "module",
        "run", "foo",
        "-d", "bar.com",
        "--user-agent", "foobar",
        "--proxy", "baz",
        "--http-timeout", "120",
        "--disable-ip-resolve",
        "--resolver-concurrency", "100",
        "--resolver-timeout", "10",
        "--resolver-list", "resolverlist.txt",
    ];

    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Module(sub) => match sub.command {
            subscan::cli::commands::module::ModuleSubCommands::Run(args) => {
                assert!(args.resolver_disabled);

                assert_eq!(args.domain, "bar.com");
                assert_eq!(args.name, "foo");
                assert_eq!(args.user_agent, "foobar");
                assert_eq!(args.proxy, Some("baz".into()));
                assert_eq!(args.http_timeout, 120);
                assert_eq!(args.resolver_concurrency, 100);
                assert_eq!(args.resolver_timeout, 10);
                assert_eq!(
                    args.resolver_list,
                    Some(PathBuf::from_str("resolverlist.txt").unwrap())
                )
            }
            _ => panic!("Expected ModuleSubCommands::Run"),
        },
        _ => panic!("Expected Commands::Module"),
    }
}
