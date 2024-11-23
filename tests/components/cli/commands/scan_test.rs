use clap::Parser;
use subscan::{
    cli::Cli,
    constants::{
        DEFAULT_HTTP_TIMEOUT, DEFAULT_MODULE_CONCURRENCY, DEFAULT_RESOLVER_CONCURRENCY,
        DEFAULT_RESOLVER_TIMEOUT, DEFAULT_USER_AGENT,
    },
    enums::{cache::CacheFilter, output::OutputFormat},
    types::filters::ModuleNameFilter,
};

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
            assert!(!args.resolver_disabled);

            assert_eq!(args.domain, "foo.com");
            assert_eq!(args.user_agent, DEFAULT_USER_AGENT);
            assert_eq!(args.proxy, None);
            assert_eq!(args.http_timeout, DEFAULT_HTTP_TIMEOUT.as_secs());
            assert_eq!(args.module_concurrency, DEFAULT_MODULE_CONCURRENCY);
            assert_eq!(args.modules, "*");
            assert_eq!(args.skips, "");
            assert_eq!(args.output, OutputFormat::JSON);
            assert_eq!(args.resolver_concurrency, DEFAULT_RESOLVER_CONCURRENCY);
            assert_eq!(args.resolver_timeout, DEFAULT_RESOLVER_TIMEOUT.as_secs());
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
        "--disable-ip-resolve",
        "--resolver-concurrency", "100",
        "--resolver-timeout", "10"
    ];

    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Scan(args) => {
            assert!(args.resolver_disabled);

            assert_eq!(args.domain, "foo.com");
            assert_eq!(args.user_agent, "foo");
            assert_eq!(args.proxy.unwrap(), "bar");
            assert_eq!(args.http_timeout, 120);
            assert_eq!(args.module_concurrency, 10);
            assert_eq!(args.modules, "google,yahoo");
            assert_eq!(args.skips, "commoncrawl");
            assert_eq!(args.output, OutputFormat::CSV);
            assert_eq!(args.resolver_concurrency, 100);
            assert_eq!(args.resolver_timeout, 10);
        }
        _ => panic!("Expected Commands::Scan"),
    }
}

#[tokio::test]
async fn scan_args_no_filter_test() {
    let args = vec!["subscan", "scan", "-d", "foo.com"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Scan(args) => {
            assert_eq!(args.filter(), CacheFilter::NoFilter);
        }
        _ => panic!("Expected Commands::Scan"),
    }
}

#[tokio::test]
async fn scan_args_with_module_name_filter_invalids_test() {
    let args = vec!["subscan", "scan", "-d", "foo.com", "--skips", "foo"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Scan(args) => {
            let expected = ModuleNameFilter {
                valids: vec![],
                invalids: vec!["foo".into()],
            };

            assert_eq!(args.filter(), CacheFilter::FilterByName(expected));
        }
        _ => panic!("Expected Commands::Scan"),
    }
}

#[tokio::test]
async fn scan_args_with_module_name_filter_valids_test() {
    #[rustfmt::skip]
    let args = vec![
        "subscan",
        "scan",
        "-d", "foo.com",
        "--modules", "foo",
        "--skips", "bar"
    ];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Scan(args) => {
            let expected = ModuleNameFilter {
                valids: vec!["foo".into()],
                invalids: vec!["bar".into()],
            };

            assert_eq!(args.filter(), CacheFilter::FilterByName(expected));
        }
        _ => panic!("Expected Commands::Scan"),
    }
}
