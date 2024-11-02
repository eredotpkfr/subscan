use clap::ValueEnum;
use std::fmt;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    TXT,
    CSV,
    #[default]
    JSON,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::TXT => write!(f, "TXT"),
            OutputFormat::CSV => write!(f, "CSV"),
            OutputFormat::JSON => write!(f, "JSON"),
        }
    }
}
