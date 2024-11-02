use clap::Parser;
use subscan::{
    cli::{commands::module::run::ModuleRunSubCommandArgs, Cli},
    config::{DEFAULT_HTTP_TIMEOUT, DEFAULT_USER_AGENT},
};

#[tokio::test]
async fn module_run_args_attribute_test() {
    let args = ModuleRunSubCommandArgs {
        name: "foo".into(),
        domain: "bar".into(),
        user_agent: "baz".into(),
        timeout: 120,
        proxy: None,
    };

    assert_eq!(args.name, "foo");
    assert_eq!(args.domain, "bar");
    assert_eq!(args.user_agent, "baz");
    assert_eq!(args.timeout, 120);
    assert_eq!(args.proxy, None);
}

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
                assert_eq!(args.domain, "bar.com");
                assert_eq!(args.name, "foo");
                assert_eq!(args.user_agent, DEFAULT_USER_AGENT);
                assert_eq!(args.proxy, None);
                assert_eq!(args.timeout, DEFAULT_HTTP_TIMEOUT.as_secs());
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
        "--timeout", "120",
    ];

    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        subscan::cli::commands::Commands::Module(sub) => match sub.command {
            subscan::cli::commands::module::ModuleSubCommands::Run(args) => {
                assert_eq!(args.domain, "bar.com");
                assert_eq!(args.name, "foo");
                assert_eq!(args.user_agent, "foobar");
                assert_eq!(args.proxy, Some("baz".into()));
                assert_eq!(args.timeout, 120);
            }
            _ => panic!("Expected ModuleSubCommands::Run"),
        },
        _ => panic!("Expected Commands::Module"),
    }
}
