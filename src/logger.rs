use colog::format::{CologStyle, DefaultCologStyle};
use env_logger::fmt::Formatter;
use log::{LevelFilter, Record};
use std::io::Write;

pub async fn init(level: Option<LevelFilter>) {
    let pkg_name = env!("CARGO_PKG_NAME");
    let filter = level.unwrap_or(LevelFilter::Debug);

    env_logger::builder()
        .filter_module(pkg_name, filter)
        .format(formatter)
        .init();
}

fn formatter(buf: &mut Formatter, record: &Record<'_>) -> Result<(), std::io::Error> {
    if record.target() == "subscan::banner" {
        writeln!(buf, "{}", record.args())
    } else {
        DefaultCologStyle.format(buf, record)
    }
}
