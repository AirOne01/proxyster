use clap::{builder, value_parser, Arg, ArgAction, Command, ValueHint};

fn clap_command() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_help_subcommand(true)
        .subcommands([
            Command::new("find")
                .visible_alias("f")
                .about("Scrape proxies from the internet")
                .after_help("If no output file is specified, the proxies will be output to `./proxies.txt`.\nThe default amount of proxies is 100.")
                .args([
                    Arg::new("output")
                        .long("out")
                        .short('o')
                        .help("File path to output to")
                        .num_args(1)
                        .value_name("PATH")
                        .value_hint(ValueHint::FilePath)
                        .value_parser(builder::PathBufValueParser::new())
                        .action(ArgAction::Set)
                        .conflicts_with("stdout"),
                    Arg::new("stdout")
                        .long("stdout")
                        .short('s')
                        .help("Dump proxies to standard output")
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                        .conflicts_with("output"),
                    Arg::new("amount")
                        .long("amount")
                        .short('n')
                        .help("Amount of proxies to find")
                        .num_args(1)
                        .value_name("AMOUNT")
                        .value_hint(ValueHint::Other)
                        .value_parser(value_parser!(u32).range(1..))
                        .conflicts_with_all(&["minmax", "min", "max"]),
                    Arg::new("minmax")
                        .long("minmax")
                        .short('m')
                        .help("Minimum and maximum amount of proxies to find")
                        .num_args(2)
                        .value_names(&["MIN", "MAX"])
                        .value_hint(ValueHint::Other)
                        .value_parser(value_parser!(u32).range(1..))
                        .conflicts_with_all(&["amount", "min", "max"]),
                    Arg::new("min")
                        .long("min")
                        .help("Minimum amount of proxies to find")
                        .num_args(1)
                        .value_name("MIN")
                        .value_hint(ValueHint::Other)
                        .value_parser(value_parser!(u32).range(1..))
                        .conflicts_with_all(&["amount", "minmax", "max"]),
                    Arg::new("max")
                        .long("max")
                        .help("Maximum amount of proxies to find")
                        .num_args(1)
                        .value_name("MAX")
                        .value_hint(ValueHint::Other)
                        .value_parser(value_parser!(u32).range(1..))
                        .conflicts_with_all(&["amount", "minmax", "min"]),
                ]),
            Command::new("read")
                .visible_alias("r")
                .about("Read proxies from previously saved file")
        ])
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Enable debug mode")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .global(true)
        )
}

pub fn cli() -> clap::ArgMatches {
    clap_command().get_matches()
}
