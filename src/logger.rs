use std::io::Write;

use colog::format::{CologStyle, DefaultCologStyle};
use env_logger::fmt::Formatter;
use log::{LevelFilter, Record};

use crate::constants::SUBSCAN_BANNER_LOG_TARGET;

/// Initialize logger
pub async fn init(level: Option<LevelFilter>) {
    let pkg_name = env!("CARGO_PKG_NAME");
    let filter = level.unwrap_or(LevelFilter::Debug);

    env_logger::builder().filter_module(pkg_name, filter).format(formatter).init();
}

// Custom formatter to avoid timestamp and log levels on banner log line
fn formatter(buf: &mut Formatter, record: &Record<'_>) -> Result<(), std::io::Error> {
    if record.target() == SUBSCAN_BANNER_LOG_TARGET {
        writeln!(buf, "{}", record.args())
    } else {
        DefaultCologStyle.format(buf, record)
    }
}
