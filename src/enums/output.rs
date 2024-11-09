use clap::ValueEnum;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    TXT,
    CSV,
    #[default]
    JSON,
}
