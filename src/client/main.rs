use crate::cli::cli;
use crate::scraper::scraper;

mod cli;
mod scraper;

fn main() {
    let matches = cli();

    match matches.subcommand() {
        Some(("find", sub)) => {
            scraper().unwrap();
        }
        _ => {}
    }
}
