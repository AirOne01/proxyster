use serde_derive::Deserialize;

/**
A proxy source

# Examples

(in `providers.toml`)

```toml
[[proxy]]
name = "proxy1"
sources = ["https://raw.githubusercontent.com/..."]
```
*/
#[derive(Deserialize)]
pub struct ProviderSource {
    pub url: Option<String>,
    pub selector: String,
    pub regex: Option<String>,
}

// implement From for ProviderSource
impl From<toml::Value> for ProviderSource {
    fn from(value: toml::Value) -> Self {
        ProviderSource {
            url: value.get("url").map(|v| v.as_str().unwrap().to_owned()),
            selector: value.get("selector").unwrap().as_str().unwrap().to_owned(),
            regex: value.get("regex").map(|v| v.as_str().unwrap().to_owned()),
        }
    }
}

// implement Clone for ProviderSource
impl Clone for ProviderSource {
    fn clone(&self) -> Self {
        ProviderSource {
            url: self.url.clone(),
            selector: self.selector.clone(),
            regex: self.regex.clone(),
        }
    }
}

