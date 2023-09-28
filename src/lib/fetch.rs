/**
 * fetch.rs
 * This file handles the fetching of proxies from the providers.
 */
use regex::Regex;
use scraper::{Html, Selector};

use crate::{config::read_config, filters::filter_all, provider::ProviderSource};

// Type alias
type ExpandedResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// sync proxy fetching
pub async fn fetch() -> ExpandedResult<Vec<String>> {
    let providers = Vec::from(read_config().providers);
    let client = reqwest::Client::new();
    let mut final_proxy_list: Vec<String> = Vec::new();

    for provider in providers {
        // fetch sources from provider.sources (TOML)
        let sources = provider.sources;
        let client2 = client.clone();
        // get proxies from sources
        let scraped_proxies = get_proxies(client2, sources).await?;
        let filtered_proxies = filter_all(scraped_proxies);
        final_proxy_list.extend(filtered_proxies);
    }

    Ok(final_proxy_list.clone())
}

// get the url and selector from the provider, and fetch a new url from those.
// do this recursively until we get a provider with no sources, and then return the selector content
async fn get_proxies(
    client: reqwest::Client,
    sources: Vec<ProviderSource>,
) -> Result<Vec<String>, &'static str> {
    let mut proxies = Vec::new();

    // only for the first one
    let mut url: String = match sources[0].url.as_ref() {
        Some(url) => url.to_owned(),
        None => return Err("Could not get url"),
    };
    let mut selector = sources[0].selector.clone();
    let mut regex = sources[0].regex.clone();

    for i in 0..sources.len() {
        if i != sources.len() - 1 {
            // if this is not last element, get the selector's href
            url = match get_html_href(
                client.clone(),
                url.as_str(),
                selector.as_str(),
                regex.clone(),
            )
            .await
            {
                Ok(url) => url,
                Err(_) => return Err("Could not get url"),
            };
            println!("url: {:?}", url);
            selector = sources[i + 1].selector.clone();
            break;
        }
        regex = sources[i].regex.clone();
        match get_html_text(client.clone(), url.as_str(), selector.as_str()).await {
            Ok(html_text) => match html_text {
                HtmlText::Text(text) => {
                    proxies.push(text);
                    return Ok(proxies);
                }
                HtmlText::IpList(ip_list) => {
                    proxies.extend(ip_list);
                    // print all proxies
                    println!("proxies: {:?}", proxies);
                    return Ok(proxies);
                }
            },
            Err(_) => return Err("Could not get proxy from html text"),
        };
    }

    Ok(proxies)
}

// from an url and a selector, returns the href of first element of the selector
async fn get_html_href(
    client: reqwest::Client,
    url: &str,
    selector: &str,
    regex: Option<String>,
) -> ExpandedResult<String> {
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse(selector).expect("Could not parse selector");
    let mut scanned_elements_with_href = false;
    for element in fragment.select(&selector) {
        if let Some(element_href) = element.value().attr("href") {
            if let Some(regex) = regex.clone() {
                let regex = match Regex::new(&regex[..]) {
                    Ok(regex) => regex,
                    Err(_) => return Err("Could not parse regex".into()),
                };
                if regex.is_match(element_href) {
                    return Ok(element_href.to_string());
                }
                scanned_elements_with_href = true;
            } else {
                return Ok(element_href.to_string());
            }
        }
    }
    if scanned_elements_with_href {
        Err("Could not find any element with href matching regex".into())
    } else {
        Err("Could not find any href".into())
    }
}

enum HtmlText {
    Text(String),
    IpList(Vec<String>),
}

// from an url and a selector, returns the html text of the selector
async fn get_html_text(
    client: reqwest::Client,
    url: &str,
    selector: &str,
) -> ExpandedResult<HtmlText> {
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    let fragment = Html::parse_document(&body);
    let selector = match Selector::parse(selector) {
        Ok(selector) => selector,
        Err(_) => return Err("Could not parse selector".into()),
    };
    let mut text = String::new();
    let mut ip_list = Vec::new();
    let mut index = 0;
    for element in fragment.select(&selector) {
        if index <= 2 {
            text.push_str(element.text().collect::<String>().as_str());
            ip_list.push(element.text().collect::<String>());
        } else {
            ip_list.push(element.text().collect::<String>());
        }
        index += 1;
    }
    if index > 2 {
        Ok(HtmlText::IpList(ip_list))
    } else {
        Ok(HtmlText::Text(text))
    }
}
