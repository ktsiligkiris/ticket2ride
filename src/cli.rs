use clap::{crate_authors, crate_version, App, Arg, SubCommand};

pub fn get_cli() -> App<'static, 'static> {
    App::new("ticket2ride max route finder")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tries to find the theoretical max score in ticket2ride")
        .arg(
            Arg::with_name("city")
                .short("c")
                .long("city")
                .value_name("Start")
                .help("Sets the City to start from")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("logging")
                .long("log-level")
                .require_equals(true)
                .value_name("LEVEL")
                .help("Sets the logging level of the app")
                .takes_value(true)
                .possible_values(&["info", "debug"]),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("provides game info")
                .version(crate_version!())
                .author(crate_authors!())
                .arg(
                    Arg::with_name("big")
                        .short("b")
                        .long("big-tickets")
                        .help("Output the available big tickets in the game")
                        .required_unless("normal"),
                )
                .arg(
                    Arg::with_name("normal")
                        .short("n")
                        .long("normal-tickets")
                        .help("Output the available normal tickets in the game")
                        .required_unless("big"),
                ),
        )
        .subcommand(
            SubCommand::with_name("solve")
                .about("try to find a solution")
                .version(crate_version!())
                .author(crate_authors!()),
        )
}
