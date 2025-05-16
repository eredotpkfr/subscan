use std::{
    collections::BTreeSet,
    fs::{read_to_string, remove_file},
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use csv::{Reader, StringRecord};
use serde_json::Value;
use subscan::{
    enums::output::OutputFormat,
    types::result::{item::SubscanResultItem, subscan::SubscanResult},
};

use crate::common::utils::fix_new_lines;

#[tokio::test]
async fn save_txt_test() {
    let mut result: SubscanResult = "foo.com".into();
    let subdomains = BTreeSet::from_iter([
        SubscanResultItem {
            subdomain: "bar.foo.com".into(),
            ip: None,
        },
        SubscanResultItem {
            subdomain: "baz.foo.com".into(),
            ip: Some(IpAddr::V4(Ipv4Addr::from_str("127.0.0.1").unwrap())),
        },
    ]);

    result.extend(subdomains);
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::TXT).await;
    let content = fix_new_lines(&read_to_string(name.clone()).unwrap());

    assert_eq!(content, "bar.foo.com\t\nbaz.foo.com\t127.0.0.1\n");

    remove_file(name).unwrap();
}

#[tokio::test]
async fn save_csv_test() {
    let mut result: SubscanResult = "foo.com".into();
    let item = SubscanResultItem {
        subdomain: "bar.foo.com".into(),
        ip: None,
    };

    let subdomains = BTreeSet::from_iter([item]);

    result.extend(subdomains);
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::CSV).await;

    let mut reader = Reader::from_path(name.clone()).unwrap();
    let mut record = StringRecord::new();

    reader.read_record(&mut record).unwrap();

    assert_eq!(reader.headers().unwrap().as_slice(), "subdomainip");
    assert_eq!(record.as_slice(), "bar.foo.com");

    remove_file(name).unwrap();
}

#[tokio::test]
async fn save_json_test() {
    let mut result: SubscanResult = "foo.com".into();
    let subs = BTreeSet::from_iter([
        SubscanResultItem {
            subdomain: "bar.foo.com".into(),
            ip: None,
        },
        SubscanResultItem {
            subdomain: "baz.foo.com".into(),
            ip: Some(IpAddr::V4(Ipv4Addr::from_str("127.0.0.1").unwrap())),
        },
    ]);
    let expected: Vec<Value> =
        subs.iter().filter_map(|sub| serde_json::to_value(sub).ok()).collect();

    result.extend(subs.clone());
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::JSON).await;
    let json: Value = serde_json::from_str(&read_to_string(name.clone()).unwrap()).unwrap();

    assert_eq!(json["metadata"]["target"], "foo.com");
    assert_eq!(json["total"], 2);
    assert_eq!(json["items"].as_array().unwrap().clone(), expected);

    remove_file(name).unwrap();
}

#[tokio::test]
async fn save_html_test() {
    let mut result: SubscanResult = "foo.com".into();
    let item = SubscanResultItem {
        subdomain: "bar.foo.com".into(),
        ip: None,
    };

    let subdomains = BTreeSet::from_iter([item]);

    result.extend(subdomains);
    result = result.with_finished().await;

    let name = result.save(&OutputFormat::HTML).await;
    let content = read_to_string(name.clone()).unwrap();

    assert!(content.contains("bar.foo.com"));

    remove_file(name).unwrap();
}
