use clap::{App, crate_version, crate_authors};
mod commands;

pub fn run() {
    let matches = App::new("meltdown")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(commands::validate::command())
        .get_matches();

    match matches.subcommand() {
        ("validate", Some(m)) => {
            if m.value_of("path").is_some() {
                println!("Validate from path: {}", m.value_of("path").unwrap());
                return;
            }

            if m.value_of("url").is_some() {
                println!("Validate from url: {}", m.value_of("url").unwrap());
                return;
            }

            println!("Provided a value for either --path or --url.");
        },
        _ => unreachable!()
    };
}