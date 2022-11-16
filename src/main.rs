use crate::cli::cli;
use crate::scraper::scraper;

mod cli;
mod scraper;

fn main() {
    cli();
    scraper().unwrap();
}
