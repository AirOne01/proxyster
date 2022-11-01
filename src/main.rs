use cli::cli;
use scrapper::scrapper;

mod cli;
mod scrapper;

fn main() {
    cli();
    scrapper().unwrap();
}
