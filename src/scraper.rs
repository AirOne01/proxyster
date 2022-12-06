use regex::Regex;
use scraper::{Html, Selector};

use proxyster_lib::util::read_config;
use proxyster_lib::provider_source::ProviderSource;

// Type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn scraper() -> Result<()> {
    let providers = Vec::from(read_config().providers);
    let client = reqwest::Client::new();

    // for each provider execute get_proxies
    for provider in providers {
        // fetch sources from provider.sources (TOML)
        let sources = provider.sources;
        // print sources urls
        let proxies = get_proxies(&client, sources).await?;
        // print a vector of proxies
        println!("{:?}", proxies);
    }

    Ok(())
}

// from an url and a selector, returns the html text of the selector
async fn get_html_text(client: &reqwest::Client, url: &str, selector: &str) -> Result<String> {
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse(selector).unwrap();
    let mut text = String::new();
    for element in fragment.select(&selector) {
        text.push_str(element.text().collect::<String>().as_str());
    }
    Ok(text)
}

// from an url and a selector, returns the href of first element of the selector
async fn get_html_href(client: &reqwest::Client, url: &str, selector: &str, regex: Option<String>) -> Result<String> {
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse(selector).expect("Could not parse selector");
    let mut scanned_elements_with_href = false;
    for element in fragment.select(&selector) {
        if let Some(element_href) = element.value().attr("href") {
            println!("{}", element_href);
            if let Some(regex) = regex.clone() {
                if Regex::new(&regex[..]).unwrap().is_match(element_href) {
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

// get the url and selector from the provider, and fetch a new url from those.
// do this recursively until we get a provider with no sources, and then return the selector content
async fn get_proxies(client: &reqwest::Client, sources: Vec<ProviderSource>) -> Result<Vec<String>> {
    let mut proxies = Vec::new();

    let mut url: String = sources[0].url.as_ref().unwrap().to_owned(); // only for the first one
    let mut selector = sources[0].selector.clone();
    let mut regex = sources[0].regex.clone();

    for i in 0..sources.len() {
        println!("{}/{}, {}", i, sources.len(), url);

        if i != sources.len() - 1 {
            // if this is not last element, get the selector's href
            url = get_html_href(client, url.as_str(), selector.as_str(), regex.clone()).await?;
            selector = sources[i+1].selector.clone();
            regex = sources[i+1].regex.clone();
        } else {
            println!("last element {}, {}", url, selector);
            proxies.push(get_html_text(client, url.as_str(), selector.as_str()).await?);
        }
    }

    Ok(proxies)
}