use subscan::{cli::commands::brute::BruteCommandArgs, enums::output::OutputFormat};

#[tokio::test]
async fn brute_args_attribute_test() {
    let args = BruteCommandArgs {
        domain: "foo".into(),
        concurrency: 10,
        output: OutputFormat::JSON,
    };

    assert_eq!(args.domain, "foo");
    assert_eq!(args.concurrency, 10);
    assert_eq!(args.output, OutputFormat::JSON);
}
