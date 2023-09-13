use std::{fs::create_dir, path::PathBuf};

use dirs::config_dir;

/**
 The config directory path
*/
pub fn vanilla_dir_exists() -> PathBuf {
    let conf_dir_resolve = config_dir().expect("should find user config directory");
    let conf_dir = conf_dir_resolve.as_path();
    match conf_dir.try_exists() {
        Ok(false) => create_dir(conf_dir).unwrap(),
        Ok(true) => {}
        Err(err) => {
            panic!("error checking if config directory exists: {}", err);
        }
    }
    assert!(
        conf_dir.is_dir(),
        "user config directory path should be a directory and not a file"
    );
    let dir = conf_dir.join("proxyster");
    match dir.try_exists() {
        Ok(false) => create_dir(dir.clone()).unwrap(),
        Ok(true) => {}
        Err(err) => {
            panic!(
                "error checking if proxyster config directory exists: {}",
                err
            );
        }
    }
    assert!(
        dir.is_dir(),
        "proxyster config directory path should be a directory and not a file"
    );
    return dir;
}
