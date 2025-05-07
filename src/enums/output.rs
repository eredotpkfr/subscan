use std::{fs::File, sync::OnceLock};

use chrono::Utc;
use clap::ValueEnum;
use derive_more::Display;

use crate::types::result::output::OutputFile;

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

static OUTPUT_FILE: OnceLock<OutputFile> = OnceLock::new();

impl OutputFormat {
    pub async fn get_file(&self, domain: &str) -> &OutputFile {
        let inner = || {
            let file_name = self.get_file_name(domain);
            let file = File::create(&file_name).unwrap();

            (file_name, file).into()
        };

        OUTPUT_FILE.get_or_init(inner)
    }

    fn get_file_name(&self, domain: &str) -> String {
        let now = Utc::now().timestamp();

        match self {
            OutputFormat::TXT => format!("{domain}.{now}.{self}"),
            OutputFormat::CSV => format!("{domain}.{now}.{self}"),
            OutputFormat::JSON => format!("{domain}.{now}.{self}"),
            OutputFormat::HTML => format!("{domain}.{now}.{self}"),
        }
    }
}
