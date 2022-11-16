use std::array::IntoIter;

use regex::Regex;

// Type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn scraper() -> Result<()> {
    let providers = init();

    for provider in providers {
        if let (Some(i_links), Some(i_selector)) = (provider.indirect_links, provider.indirect_links_selector) {
            for i_link in i_links {
                let resp = reqwest::get(i_link)
                    .await?
                    .text()
                    .await?;
                let document = scraper::Html::parse_document(&resp);
                let selector = scraper::Selector::parse(&i_selector[..])
                    .unwrap();
                let selected: Vec<&str> = if let Some(i_regex) = provider.indirect_links_regex.clone() {
                    let reg = Regex::new(&i_regex[..])
                        .unwrap();
                    document.select(&selector)
                        .map(|x| x.value().attr("href").unwrap_or(""))
                        .filter(|x| reg.is_match(x))
                        .into_iter()
                        .collect()
                } else {
                    document.select(&selector)
                        .map(|x| x.value().attr("href").unwrap_or(""))
                        .collect()
                };
                selected
                    .iter()
                    .for_each(|e| println!("{}", e));
            }
        } else {
            panic!("No indirect links");
        }
    }

    Ok(())
}

struct Provider {
    name: String,
    indirect_links: Option<Vec<String>>,
    indirect_links_selector: Option<String>,
    indirect_links_regex: Option<String>,
    direct_links: Option<Vec<String>>,
    direct_links_selector: Option<String>,
    direct_links_regex: Option<Vec<String>>,
}

impl Default for Provider {
    fn default() -> Self {
        Self {
            name: Default::default(),
            indirect_links: Default::default(),
            indirect_links_selector: Default::default(),
            indirect_links_regex: Default::default(),
            direct_links: Default::default(),
            direct_links_selector: Default::default(),
            direct_links_regex: Default::default(),
        }
    }
}

fn init() -> Vec<Provider> {
    Vec::from([Provider {
        name: String::new(),
        indirect_links: Some(Vec::from(["http://www.socks24.org".to_string()])),
        indirect_links_selector: Some("a".to_string()),
        // indirect_links_regex: None,
        indirect_links_regex: Some(String::from(r"^http://www\.socks24\.org/\d\d\d\d/\d\d/\d\d\-\d\d\-\d\d\-(?:vip\-socks\-5_\d\d|us\-socks(?:_\d\d)?)\.html$")),
        direct_links: None,
        direct_links_selector: None,
        direct_links_regex: None
    }])
}
