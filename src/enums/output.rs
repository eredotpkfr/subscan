use std::fs::File;

use chrono::Utc;
use clap::ValueEnum;
use derive_more::Display;

/// Supported output formats for reporting scan results
#[derive(Copy, Clone, Debug, Default, Display, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum OutputFormat {
    #[display("txt")]
    TXT,
    #[display("csv")]
    CSV,
    #[default]
    #[display("json")]
    JSON,
    #[display("html")]
    HTML,
}

impl OutputFormat {
    pub async fn get_file(&self, domain: &str) -> (File, String) {
        let now = Utc::now().timestamp();
        let filename = match self {
            OutputFormat::TXT => format!("{domain}.{now}.{self}"),
            OutputFormat::CSV => format!("{domain}.{now}.{self}"),
            OutputFormat::JSON => format!("{domain}.{now}.{self}"),
            OutputFormat::HTML => format!("{domain}.{now}.{self}"),
        };

        (File::create(filename.clone()).unwrap(), filename)
    }
}
