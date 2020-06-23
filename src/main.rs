use clap::{App, Arg, SubCommand};

mod build;
mod config;
mod create;

fn main() {
    let matches = App::new("modcrafter")
        .about("A tool for creating forge mods for minecraft")
        .author("Ian Rehwinkel <ian.rehwinkel@tutanota.com>")
        .version("1.0")
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new project")
                .arg(
                    Arg::with_name("DIR")
                        .help("The directory of the new project")
                        .required(true),
                )
                .arg(
                    Arg::with_name("FORGE")
                        .help("The path to the forge MDK zip file")
                        .required(true),
                )
                .arg(
                    Arg::with_name("display")
                        .long("display-name")
                        .value_name("NAME")
                        .help("The display name of the mod"),
                )
                .arg(
                    Arg::with_name("modid")
                        .long("modid")
                        .value_name("ID")
                        .help("The mod ID"),
                )
                .arg(
                    Arg::with_name("version")
                        .long("version")
                        .value_name("VERSION")
                        .help("The version of the mod"),
                )
                .arg(
                    Arg::with_name("description")
                        .long("description")
                        .value_name("DESC")
                        .help("The mod description"),
                ),
        )
        .get_matches();
    let result = match matches.subcommand() {
        ("create", Some(extra)) => create::create_project(create::Parameters::new(
            extra.value_of("DIR").unwrap(),
            extra.value_of("FORGE").unwrap(),
            extra.value_of("display"),
            extra.value_of("modid"),
            extra.value_of("version"),
            extra.value_of("description"),
            extra.value_of("authors"),
        )),
        _ => return,
    };

    if let Err(err) = result {
        println!("{}", err);
    }
}
