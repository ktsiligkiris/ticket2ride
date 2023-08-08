use clap::{crate_authors, crate_version, Arg, Command};

pub fn get_cli() -> Command {
    Command::new("ticket2ride max route finder")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tries to find the theoretical max score in ticket2ride")
        .arg(
            Arg::new("city")
                .short('c')
                .long("city")
                .value_name("Start")
                .default_value("Edinburgh")
                .help("Sets the City to start from"),
        )
        .arg(
            Arg::new("logging")
                .long("log-level")
                .default_value("none")
                .require_equals(true)
                .value_name("LEVEL")
                .help("Sets the logging level of the app")
                .value_parser(["info", "debug", "none"]),
        )
        .subcommand(
            Command::new("info")
                .about("provides game info")
                .version(crate_version!())
                .author(crate_authors!())
                .arg(
                    Arg::new("big")
                        .short('b')
                        .long("big-tickets")
                        .help("Output the available big tickets in the game")
                        .required_unless_present("normal"),
                )
                .arg(
                    Arg::new("normal")
                        .short('n')
                        .long("normal-tickets")
                        .help("Output the available normal tickets in the game")
                        .required_unless_present("big"),
                ),
        )
        .subcommand(
            Command::new("solve")
                .about("try to find a solution")
                .version(crate_version!())
                .author(crate_authors!()),
        )
}
