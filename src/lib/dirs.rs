use std::path::PathBuf;

use dirs::config_dir;

/**
 The config directory path
*/
pub fn vanilla_dir_exists() -> PathBuf {
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
    return dir;
}