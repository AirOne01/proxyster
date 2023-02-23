use dirs::config_dir;
use std::fs::read_to_string;

use crate::config::Config;

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

