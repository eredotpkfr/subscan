use regex::Regex;
use reqwest;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashSet;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Google {
    url: &'static str,
    domain: String,
    client: reqwest::Client,
}

impl Google {
    pub async fn new(domain: String) -> Google {
        Google {
            url: "https://www.google.com/search",
            domain: domain,
            client: reqwest::Client::new(),
        }
    }

    pub async fn start(&self) {
        let mut query = format!("site:{}", self.domain);
        let mut all_results = HashSet::new();

        loop {
            let req = self
                .client
                .get(self.url)
                .header("User-Agent", USER_AGENT)
                .query(&[("q", query.clone()), ("num", 50.to_string())])
                .build()
                .unwrap();

            let response = self.client.execute(req).await.unwrap();

            if response.status() != 200 {
                return;
            }

            let content = response.text().await.unwrap();

            let document = Html::parse_document(&content);
            let cite_selector = Selector::parse("cite").unwrap();

            let pattern = format!(
                r"([a-zA-Z0-9.]+)\.{}$",
                self.domain.clone().replace(".", r"\.")
            );
            let re = Regex::new(&pattern).unwrap();

            let cites: Vec<ElementRef<'_>> = document.select(&cite_selector).collect();
            let page_results: HashSet<String> = cites
                .iter()
                .filter_map(|item| {
                    if let Some(matches) = re.find(&item.inner_html()) {
                        Some(matches.as_str().to_string())
                    } else {
                        None
                    }
                })
                .collect();

            all_results.extend(page_results.clone());

            let new_queries: Vec<String> = page_results
                .iter()
                .filter_map(|item| {
                    let formatted = &format!(".{}", self.domain);

                    if let Some(sub) = item.strip_suffix(formatted) {
                        Some(format!("-{}", sub).to_string())
                    } else {
                        None
                    }
                })
                .collect();

            let new_query = format!("{} {}", query, new_queries.join(" "))
                .trim()
                .to_string();

            if query == new_query {
                break;
            } else {
                query = new_query;
            }
        }
        println!("{:#?}\n{}", all_results, all_results.len());
    }
}
