use std::fs::File;

use chrono::Utc;
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

impl OutputFormat {
    pub async fn get_file(&self, domain: &str) -> (File, String) {
        let now = Utc::now().timestamp();
        let filename = match self {
            OutputFormat::TXT => format!("{}_{}.txt", domain, now),
            OutputFormat::CSV => format!("{}_{}.csv", domain, now),
            OutputFormat::JSON => format!("{}_{}.json", domain, now),
            OutputFormat::HTML => format!("{}_{}.html", domain, now),
        };

        (File::create(filename.clone()).unwrap(), filename)
    }
}
