use crate::cli::cli;
use crate::scraper::scraper;

mod cli;
mod scraper;

fn main() {
    let mut _dump_to_stdout = false;
    let mut _debug = false;
    let matches = cli();

    match matches.subcommand() {
        Some(("stdout", _sub)) => {
            _dump_to_stdout = true;
        }
        Some(("find", _sub)) => {
            scraper(_dump_to_stdout, _debug).unwrap();
        }
        _ => {}
    }
}
