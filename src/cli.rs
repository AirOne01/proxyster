use clap::Command;

fn clap_command() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("find").about("Find proxies"))
}

pub fn cli() {
    match clap_command().get_matches().subcommand() {
        Some(("find", _sub)) => {}
        _ => {}
    }
}
