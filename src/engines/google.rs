use crate::utils::regex;
use reqwest::Client;
use scraper::html::Select;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::iter::FilterMap;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Google {
    url: &'static str,
    domain: String,
    client: Client,
}

impl Google {
    pub async fn new(domain: String) -> Google {
        Google {
            url: "https://www.google.com/search",
            domain: domain,
            client: Client::new(),
        }
    }

    pub async fn start(&self) {
        let mut query = format!("site:{}", self.domain);
        let mut all_results = HashSet::new();

        loop {
            let request = self
                .client
                .get(self.url)
                .header("User-Agent", USER_AGENT)
                .query(&[("q", query.clone()), ("num", 100.to_string())])
                .build()
                .unwrap();

            let response = self.client.execute(request).await.unwrap();

            if response.status() != 200 {
                break;
            }

            let content = response.text().await.unwrap();

            let document = Html::parse_document(&content);
            let cite_selector = Selector::parse("cite").unwrap();

            let pattern = regex::generate_domain_regex(self.domain.clone()).unwrap();

            let page_results: FilterMap<Select<'_, '_>, _> =
                document.select(&cite_selector).filter_map(|item| {
                    if let Some(matches) = pattern.find(&item.inner_html()) {
                        Some(matches.as_str().to_string())
                    } else {
                        None
                    }
                });

            all_results.extend(page_results.clone());

            let new_queries: Vec<String> = page_results
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
            }

            query = new_query;
        }
        println!("{:#?}\n{}", all_results, all_results.len());
    }
}
