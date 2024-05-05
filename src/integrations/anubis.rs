use reqwest::redirect::Policy;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Anubis {
    url: &'static str,
    domain: String,
    client: Client,
}

impl Anubis {
    pub async fn new(domain: String) -> Anubis {
        Anubis {
            url: "https://jonlu.ca/anubis/subdomains/",
            domain: domain,
            client: Client::new(),
        }
    }

    pub async fn start(&self) {
        let request = self
            .client
            .get(format!("{}{}", self.url, self.domain))
            .header("User-Agent", USER_AGENT)
            .build()
            .unwrap();

        let response = self.client.execute(request).await.unwrap();

        if response.status() != 200 && response.status() != 300 {
            return;
        }

        let content = response.text().await.unwrap();
        let res: Value = serde_json::from_str(&content).unwrap();

        if let Some(subs) = res.as_array() {
            let all_results: HashSet<String> = subs
                .iter()
                .filter_map(|item| Some(item.as_str()?.to_string()))
                .collect();
            println!("{:#?}\n{}", all_results, all_results.len());
        }
    }
}
