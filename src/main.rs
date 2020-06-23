use clap::{App, Arg, SubCommand};
use std::fmt::{self, Display, Formatter};
use std::io::{self};
use zip::result::ZipError;

mod config;
mod create;

#[derive(Debug)]
pub enum Error {
    ProjectFolderExists,
    FileIOError,
    ZipIOError,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::FileIOError
    }
}

impl From<ZipError> for Error {
    fn from(_: ZipError) -> Self {
        Error::ZipIOError
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
                ),
        )
        .get_matches();
    let result = match matches.subcommand() {
        ("create", Some(extra)) => create::create_project(
            extra.value_of("DIR").unwrap(),
            extra.value_of("FORGE").unwrap(),
            None,
            None,
            None,
            None,
        ),
        _ => panic!("invalid subcommand"),
    };

    if let Err(err) = result {
        println!("{}", err);
    }
}
