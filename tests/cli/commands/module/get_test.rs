use clap::Parser;
use std::io::Cursor;
use subscan::{cli::Cli, modules::engines::google::Google};
use tokio::sync::Mutex;

#[tokio::test]
#[should_panic]
async fn module_get_parse_error_test() {
    Cli::try_parse_from(vec!["subscan", "module", "get", "-x"]).unwrap();
}

#[tokio::test]
async fn module_get_test() {
    let args = vec!["subscan", "module", "get", "google"];
    let cli = Cli::try_parse_from(args).unwrap();

    let expected = "\
        +--------+------------+---------------+-------------+\n\
        | Name   | Requester  | Extractor     | Is Generic? |\n\
        +--------+------------+---------------+-------------+\n\
        | google | HTTPClient | HTMLExtractor | true        |\n\
        +--------+------------+---------------+-------------+\n\
    ";

    match cli.command {
        subscan::cli::commands::Commands::Module(sub) => match sub.command {
            subscan::cli::commands::module::ModuleSubCommands::Get(args) => {
                let mut out = Cursor::new(vec![]);
                let module = Mutex::new(Google::dispatcher());

                args.as_table(&module, &mut out).await;

                let result = String::from_utf8(out.into_inner()).unwrap();

                assert_eq!(args.name, "google");
                assert_eq!(result, expected);
            }
            _ => panic!("Expected ModuleSubCommands::Get"),
        },
        _ => panic!("Expected Commands::Module"),
    }
}
