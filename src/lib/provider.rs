use serde_derive::Deserialize;

/**
A proxy provider

# Examples

(in `providers.toml`)

```toml
[[proxy]]
name = "proxy1"
sources = ["https://raw.githubusercontent.com/..."]
```
*/
#[derive(Deserialize)]
pub struct Provider {
    pub name: String,
    pub sources: Vec<ProviderSource>, // array of InitialProviderSource
}

// implement From for Provider
impl From<toml::Value> for Provider {
    fn from(value: toml::Value) -> Self {
        let sources: Vec<ProviderSource> = value
            .get("sources")
            .expect("sources should be defined")
            .as_array()
            .expect("sources should be converted to an array")
            .iter()
            .map(|v| ProviderSource::from(v.clone()))
            .collect();
        let provider_name = value
            .get("name")
            .expect("name should be defined")
            .to_string();

        Provider {
            name: provider_name,
            sources,
        }
    }
}

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
