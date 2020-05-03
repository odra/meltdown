use clap::{App, Arg};

pub fn command<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("validate")
        .about("validate an spec")
        .arg(
            Arg::with_name("path")
            .short("p")
            .long("path")
            .help("file path to load spec from")
            .required(false)
            .takes_value(true)
        )
        .arg(
            Arg::with_name("url")
            .short("u")
            .long("url")
            .help("url to load spec from")
            .required(false)
            .takes_value(true)
        )
}