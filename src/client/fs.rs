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

pub fn read_proxies() -> Result<(), &'static str> {
    let dir = vanilla_dir_exists();
    let proxies_file = dir.join("proxies.txt");
    if proxies_file.exists() {
        match proxies_file.is_file() {
            true => {}
            false => {
                return Err("proxies file path should be a file and not a directory");
            }
        }
    };
    let proxies = match std::fs::read_to_string(proxies_file) {
        Ok(proxies) => proxies,
        Err(_) => {
            return Err("Could not read proxies");
        }
    };
    println!("{}", proxies);

    Ok(())
}
