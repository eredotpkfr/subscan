use clap::ValueEnum;

/// Supported output formats for reporting scan results
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    TXT,
    CSV,
    #[default]
    JSON,
    HTML,
}
