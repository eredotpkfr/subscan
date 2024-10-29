use clap::ValueEnum;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    #[default]
    TXT,
    CSV,
    JSON,
}
