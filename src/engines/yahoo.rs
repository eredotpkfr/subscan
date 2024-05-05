use crate::utils::regex;
use reqwest::Client;
use scraper::{html::Select, Html, Selector};
use std::collections::HashSet;
use std::iter::FilterMap;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Yahoo {
    url: &'static str,
    domain: String,
    client: Client,
}

impl Yahoo {
    pub async fn new(domain: String) -> Yahoo {
        Yahoo {
            url: "https://search.yahoo.com/search",
            domain: domain,
            client: Client::new(),
        }
    }

    pub async fn start(&self) {
        let mut query = format!("site:{}", self.domain);
        let mut all_results: HashSet<String> = HashSet::new();
        let mut query_state: HashSet<String> = HashSet::new();

        loop {
            let request = self
                .client
                .get(self.url)
                .header("User-Agent", USER_AGENT)
                .query(&[("p", query.clone())])
                .build()
                .unwrap();

            let response = self.client.execute(request).await.unwrap();

            if response.status() != 200 {
                break;
            }

            let content = response.text().await.unwrap();

            let document = Html::parse_document(&content);
            let cite_selector = Selector::parse("ol > li > div > div > h3 > a > span").unwrap();

            let pattern = regex::generate_domain_regex(self.domain.clone()).unwrap();

            let page_results: FilterMap<Select<'_, '_>, _> =
                document.select(&cite_selector).filter_map(|item| {
                    let inner_text = item.inner_html().replace("<b>", "").replace("</b>", "");

                    if let Some(matches) = pattern.find(&inner_text) {
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
                        if !query_state.contains(sub) {
                            query_state.insert(sub.to_string());
                            Some(format!("-{}", sub).to_string())
                        } else {
                            None
                        }
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
            println!("query: {}", query);
        }
        println!("{:#?}\n{}", all_results, all_results.len());
    }
}
