use std::{env, io::Write};

use subscan::{types::config::subscan::SubscanConfig, Subscan};
use tempfile::NamedTempFile;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut wordlist = NamedTempFile::new().unwrap();

    writeln!(wordlist, "api").unwrap();
    writeln!(wordlist, "app").unwrap();
    writeln!(wordlist, "test").unwrap();

    let config = SubscanConfig {
        wordlist: Some(wordlist.path().to_path_buf()),
        ..Default::default()
    };

    let subscan = Subscan::from(config);
    let result = subscan.brute(&args[1]).await;

    for item in result.items {
        println!("{}", item.as_txt())
    }
}
