use std::fs::{read_to_string};

use serde_derive::Deserialize;

use crate::{provider::Provider, dirs::vanilla_dir_exists};

/**
 The config file
*/
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
    let dir = vanilla_dir_exists();
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
