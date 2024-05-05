use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct AlienVault {
    url: &'static str,
    domain: String,
    client: Client,
}

impl AlienVault {
    pub async fn new(domain: String) -> AlienVault {
        AlienVault {
            url: "https://otx.alienvault.com/api/v1/indicators/domain/",
            domain: domain,
            client: Client::new(),
        }
    }

    pub async fn start(&self) {
        let mut all_results: HashSet<String> = HashSet::new();

        let request = self
            .client
            .get(format!("{}{}{}", self.url, self.domain, "/passive_dns"))
            .header("User-Agent", USER_AGENT)
            .build()
            .unwrap();

        let response = self.client.execute(request).await.unwrap();

        if response.status() != 200 {
            return;
        }

        let content = response.text().await.unwrap();
        let res: Value = serde_json::from_str(&content).unwrap();

        if let Some(passives) = res["passive_dns"].as_array() {
            all_results.extend(
                passives
                    .iter()
                    .filter_map(|item| Some(item["hostname"].as_str()?.to_string())),
            );
        }
        println!("{:#?}\n{}", all_results, all_results.len());
    }
}
