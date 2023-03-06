use lib::dirs::vanilla_dir_exists;
use std::fs::write;

pub fn write_proxies(data: String) -> Result<(), std::io::Error> {
    let dir = vanilla_dir_exists();
    let proxies_file = dir.join("proxies.txt");
    if proxies_file.exists() {
        assert!(
            proxies_file.is_file(),
            "proxies file path should be a file and not a directory"
        );
    };
    write(proxies_file, data)?;
    Ok(())
}
