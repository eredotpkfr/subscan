use clap::Parser;
use std::io::Cursor;
use subscan::{cli::Cli, modules::engines::google::Google, types::core::SubscanModule};

#[tokio::test]
#[should_panic]
async fn module_list_parse_error_test() {
    Cli::try_parse_from(vec!["subscan", "module", "list", "-x"]).unwrap();
}

#[tokio::test]
async fn module_list_test() {
    let args = vec!["subscan", "module", "list"];
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
            subscan::cli::commands::module::ModuleSubCommands::List(args) => {
                let mut out = Cursor::new(vec![]);
                let module = SubscanModule::from(Google::dispatcher());

                args.as_table(&vec![module], &mut out).await;

                let result = String::from_utf8(out.into_inner()).unwrap();

                assert_eq!(result, expected);
            }
            _ => panic!("Expected ModuleSubCommands::List"),
        },
        _ => panic!("Expected Commands::Module"),
    }
}
