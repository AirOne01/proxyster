use regex::Regex;
use select::document::Document;
use select::predicate::Name;

// Type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn scrapper() -> Result<()> {
    let resp = reqwest::get("http://www.socks24.org/")
        .await?
        .text()
        .await?;
    Document::from(resp.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|x| {
            Regex::new(
                r"^http://www\.socks24\.org/\d\d\d\d/\d\d/\d\d\-\d\d\-\d\d\-(?:vip\-socks\-5_\d\d|us\-socks(?:_\d\d)?)\.html$",
            )
            .unwrap()
            .is_match(x)
        })
        .for_each(|e| println!("{}", e));

    Ok(())
}
