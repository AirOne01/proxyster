use clap::{Command, arg, ArgAction::SetTrue, ArgMatches};

fn clap_command() -> Command {
    Command::new("proxysterd")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .disable_help_subcommand(true)
        .args([
            arg!(-v --verbose)
                .action(SetTrue)
                .help("Enable verbose mode")
                .conflicts_with("quiet"),
            arg!(-q --quiet)
                .action(SetTrue)
                .help("Disable standard output")
                .conflicts_with("verbose"),
        ])
}

pub fn cli() -> ArgMatches {
    clap_command().get_matches()
}
