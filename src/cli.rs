use clap::{crate_authors, crate_version, App, Arg};

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
}
