use crate::cli::cli;
use crate::scraper::scraper;

mod cli;
mod scraper;
mod filters;

fn main() {
    let matches = cli();

    match matches.subcommand() {
        Some(("find", sub)) => {
            scraper(sub.get_flag("stdout"), matches.get_flag("debug")).unwrap();
        }
        _ => {}
    }
}
