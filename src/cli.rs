use clap::{Command, Arg, ArgAction, ValueHint, builder};

fn clap_command() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("find")
                .about("Find proxies")
                .arg(
                    Arg::new("output")
                        .long("out")
                        .short('o')
                        .help("Specify the file path to output to. Default is `./proxies.txt`")
                        .num_args(1)
                        .value_name("PATH")
                        .value_hint(ValueHint::FilePath)
                        .value_parser(builder::PathBufValueParser::new())
                        .action(ArgAction::Set)
                        .conflicts_with("stdout")
                )
                .arg(
                    Arg::new("stdout")
                        .long("stdout")
                        .short('s')
                        .help("Dump proxies to standard output")
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                        .conflicts_with("output")
                )
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Enable debug mode")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
}

pub fn cli() -> clap::ArgMatches {
    return clap_command().get_matches();
}
