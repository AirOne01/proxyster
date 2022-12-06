use std::fs::read_to_string;

use dirs::config_dir;
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

// implement From for Providerpo
impl From<toml::Value> for Provider {
    fn from(value: toml::Value) -> Self {
        let sources: Vec<ProviderSource> = value
            .get("sources")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| ProviderSource::from(v.clone()))
            .collect();

        Provider {
            name: value.get("name").unwrap().as_str().unwrap().to_owned(),
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

#[derive(Deserialize)]
pub struct Config {
    pub providers: Vec<Provider>,
}

/**
 Reads the config file and returns the providers

 Runs the following tests:
    - generic config directory path exists
    - generic config directory path is a directory
    - proxyster config directory path exists
    - proxyster config directory path is a directory
    - proxyster providers file path exists
    - proxyster providers file path is a file

 Then it reads the file and returns the providers
*/
pub fn read_config() -> Config {
    let conf_dir_resolve = config_dir().expect("should find user config directory");
    let conf_dir = conf_dir_resolve.as_path();
    assert!(conf_dir.exists(), "user config directory should exist");
    assert!(
        conf_dir.is_dir(),
        "user config directory path should be a directory and not a file"
    );
    let dir = conf_dir.join("proxyster");
    assert!(dir.exists(), "proxyster config directory should exist");
    assert!(
        dir.is_dir(),
        "proxyster config directory path should be a directory and not a file"
    );
    let providers_file = dir.join("providers.toml");
    assert!(
        providers_file.exists(),
        "providers config file should exist"
    );
    assert!(
        providers_file.is_file(),
        "providers config file path should be a file and not a directory"
    );
    toml::from_str(&read_to_string(providers_file).unwrap()[..]).unwrap()
}