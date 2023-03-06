use crate::cli::cli;
use crate::scraper::scraper;
use crate::fs::read_proxies;

mod cli;
mod scraper;
mod fs;

fn main() {
    let matches = cli();

    match matches.subcommand() {
        Some(("find", _sub)) => {
            scraper().unwrap();
        },
        Some(("read", _sub)) => {
            println!("{}", read_proxies().expect("could not read proxies file"));
        }
        _ => {}
    }
}
