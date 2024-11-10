use crate::common::funcs::fix_new_lines;
use csv::{Reader, StringRecord};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs::{read_to_string, remove_file},
};
use subscan::{enums::output::OutputFormat, types::result::scan::ScanResult};

#[tokio::test]
async fn save_txt_test() {
    let mut result: ScanResult = "foo.com".into();
    let subs = BTreeSet::from_iter(["bar.foo.com".into()]);

    result.extend(subs);
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::TXT).await;
    let content = fix_new_lines(&read_to_string(name.clone()).unwrap());

    assert_eq!(content, "bar.foo.com\n");

    remove_file(name).unwrap();
}

#[tokio::test]
async fn save_csv_test() {
    let mut result: ScanResult = "foo.com".into();
    let subs = BTreeSet::from_iter(["bar.foo.com".into()]);

    result.extend(subs);
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::CSV).await;

    let mut reader = Reader::from_path(name.clone()).unwrap();
    let mut record = StringRecord::new();

    reader.read_record(&mut record).unwrap();

    assert_eq!(reader.headers().unwrap().as_slice(), "subdomains");
    assert_eq!(record.as_slice(), "bar.foo.com");

    remove_file(name).unwrap();
}

#[tokio::test]
async fn save_json_test() {
    let mut result: ScanResult = "foo.com".into();
    let subs = BTreeSet::from_iter(["bar.foo.com".into()]);

    result.extend(subs);
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::JSON).await;
    let json: Value = serde_json::from_str(&read_to_string(name.clone()).unwrap()).unwrap();

    assert_eq!(json["metadata"]["target"], "foo.com");
    assert_eq!(json["total"], 1);
    assert_eq!(
        json["results"].as_array().unwrap().clone(),
        vec!["bar.foo.com"]
    );

    remove_file(name).unwrap();
}

#[tokio::test]
async fn save_html_test() {
    let mut result: ScanResult = "foo.com".into();
    let subs = BTreeSet::from_iter(["bar.foo.com".into()]);

    result.extend(subs);
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::HTML).await;
    let content = read_to_string(name.clone()).unwrap();

    assert!(content.contains("bar.foo.com"));

    remove_file(name).unwrap();
}