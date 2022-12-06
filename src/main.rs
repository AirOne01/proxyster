use crate::cli::cli;
use crate::scraper::scraper;

mod cli;
mod scraper;
mod config;

fn main() {
    cli();
    scraper().unwrap();
}
