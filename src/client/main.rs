use crate::cli::cli;
use crate::scraper::scraper;

mod cli;
mod scraper;
mod fs;

fn main() {
    let matches = cli();

    match matches.subcommand() {
        Some(("find", _sub)) => {
            scraper().unwrap();
        }
        _ => {}
    }
}
