use clap::ArgMatches;
use lib::log::make_logger;
use slog::error;

use crate::cli::cli;
use crate::fs::read_proxies;
use crate::scraper::scraper;

mod cli;
mod fs;
mod scraper;

fn main() {
    let logger = make_logger();

    let matches = cli();

    match subcommand(matches) {
        Ok(_) => {}
        Err(e) => {
            error!(logger, "{}", e);
        }
    }
}

fn subcommand(matches: ArgMatches) -> Result<(), &'static str> {
    match matches.subcommand() {
        Some(("find", _)) => scraper(),
        Some(("read", _)) => match read_proxies() {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err("Could not read proxies");
            }
        },
        _ => {
            return Err("No subcommand was used");
        }
    }?;

    Ok(())
}
