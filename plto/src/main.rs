// std
use std::fs::read_to_string;

// crates
use clap::{App, Arg, SubCommand};
use pluto::{validate, PlutoParser, Rule};

// local

fn main() {
    let app = App::new("plto")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("validate")
                .about("Validate a given pluto file")
                .arg(Arg::with_name("INPUT").help("The file to validate")),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("validate") {
        let file = v.value_of("INPUT").unwrap();
        let text = read_to_string(file).unwrap();
        match validate(&text) {
            Ok(()) => println!("{} is valid!", file),
            Err(errors) => {
                for i in errors.iter() {
                    println!("{}", i);
                }
            }
        };
    }
}
